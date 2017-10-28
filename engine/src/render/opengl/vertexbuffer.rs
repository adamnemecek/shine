use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;

use arrayvec::ArrayVec;

use render::*;
use render::opengl::lowlevel::*;
use render::opengl::lowlevel::vertexbinding::*;

/// Structure to store reference to a single attribute os a buffer
#[derive(Clone)]
pub struct GLVertexAttributeRef {
    target: Rc<RefCell<GLVertexBufferData>>,
    attribute_index: usize
}

impl GLVertexAttributeRef {
    pub fn bind(&self, ll: &mut LowLevel, location: GLuint) {
        let vb = self.target.borrow();
        vb.bind(ll, location, self.attribute_index);
    }
}

pub type GLVertexBufferAttributeVec = ArrayVec<[GLVertexBufferAttribute; MAX_VERTEX_ATTRIBUTE_COUNT]>;


/// Structure to store hardware data associated to a VertexBuffer.
struct GLVertexBufferData {
    hw_id: GLuint,
    attributes: GLVertexBufferAttributeVec,
}

impl GLVertexBufferData {
    fn new() -> GLVertexBufferData {
        GLVertexBufferData {
            hw_id: 0,
            attributes: GLVertexBufferAttributeVec::new(),
        }
    }

    fn upload_data<VD: VertexDeclaration>(&mut self, ll: &mut LowLevel, data: &[u8]) {
        for idx in VD::get_attributes() {
            self.attributes.push(VD::get_attribute_layout(*idx));
            assert!(self.attributes.len() <= MAX_VERTEX_ATTRIBUTE_COUNT, "Vertex attribute count exceeds engine limits ({})", MAX_VERTEX_ATTRIBUTE_COUNT);
        }

        gl_check_error();
        if self.hw_id == 0 {
            unsafe {
                gl::GenBuffers(1, &mut self.hw_id);
            }
        }
        assert!(self.hw_id != 0);

        ll.vertex_binding.bind_buffer(self.hw_id);
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                           data.len() as GLsizeiptr,
                           data.as_ptr() as *const GLvoid,
                           gl::STATIC_DRAW);
        }
        gl_check_error();
    }

    fn bind(&self, ll: &mut LowLevel, location: GLuint, attribute: usize) {
        ll.vertex_binding.bind_attribute(location, self.hw_id, &self.attributes[attribute as usize]);
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.vertex_binding.unbind_if_active(self.hw_id);
        unsafe {
            gl::DeleteBuffers(1, &self.hw_id);
        }
        self.hw_id = 0;
    }
}

impl Drop for GLVertexBufferData {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking vertex buffer");
    }
}


/// RenderCommand to create and allocated OpenGL resources.
struct CreateCommand<VD: VertexDeclaration> {
    target: Rc<RefCell<GLVertexBufferData>>,
    data: Vec<u8>,
    phantom: PhantomData<VD>,
}

impl<VD: VertexDeclaration> Command for CreateCommand<VD> {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data::<VD>(ll, self.data.as_slice());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Rc<RefCell<GLVertexBufferData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}


/// VertexBuffer implementation for OpenGL.
pub struct GLVertexBuffer(Rc<RefCell<GLVertexBufferData>>);

impl GLVertexBuffer {
    pub fn new() -> GLVertexBuffer {
        GLVertexBuffer(
            Rc::new(RefCell::new(GLVertexBufferData::new()))
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        //println!("GLVertexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<VD: VertexDeclaration, Q: CommandQueue>(&mut self, queue: &mut Q, vertex_data: &[u8]) {
        //println!("GLVertexBuffer - set_copy");
        queue.add(
            CreateCommand::<VD> {
                target: self.0.clone(),
                data: vertex_data.to_vec(),
                phantom: PhantomData,
            }
        );
    }

    pub fn get_attribute_ref(&self, attribute_index: usize) -> GLVertexAttributeRef {
        assert!(attribute_index < MAX_VERTEX_ATTRIBUTE_COUNT);
        GLVertexAttributeRef {
            target: self.0.clone(),
            attribute_index: attribute_index,
        }
    }
}


/// Describe the memory layout of a vertex attribute
pub type VertexBufferLayoutElementImpl = GLVertexBufferAttribute;

/// The vertex buffer implementation
pub type VertexBufferImpl = GLVertexBuffer;

/// Reference to an attribute in a vertex buffer
pub type VertexAttributeRefImpl = GLVertexAttributeRef;

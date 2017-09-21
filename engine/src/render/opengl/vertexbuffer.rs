use std::rc::Rc;
use std::cell::RefCell;

use arrayvec::ArrayVec;

use render::*;
use render::opengl::lowlevel::*;

/// Structure to store reference to a single attribute os a buffer
pub struct GLVertexAttribute {
    target: Rc<RefCell<GLVertexBufferData>>,
    attribute_index: usize
}

impl GLVertexAttribute {
    pub fn bind(&self, ll: &mut LowLevel, location: GLuint) {
        let vb = self.target.borrow();
        vb.bind(ll, location, self.attribute_index);
    }
}

pub type GLVertexAttributeVec = ArrayVec<[GLVertexAttribute; MAX_BOUND_ATTRIBUTE_COUNT]>;
pub type GLVertexAttributeDescriptorVec = ArrayVec<[GLVertexAttributeDescriptor; MAX_VERTEX_ATTRIBUTE_COUNT]>;


/// Structure to store hardware data associated to a VertexBuffer.
struct GLVertexBufferData {
    hw_id: GLuint,
    attributes: GLVertexAttributeDescriptorVec,
}

impl GLVertexBufferData {
    fn new() -> GLVertexBufferData {
        GLVertexBufferData {
            hw_id: 0,
            attributes: GLVertexAttributeDescriptorVec::new(),
        }
    }

    fn upload_data(&mut self, ll: &mut LowLevel, attributes: &GLVertexAttributeDescriptorVec, data: &[u8]) {
        self.attributes = attributes.clone();

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
        ll.vertex_binding.delayed_bind_attribute(location, self.hw_id, &self.attributes[attribute as usize]);
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


/// RenderCommand to create the OpenGL program, set the shader sources and compile (link) a shader program.
struct CreateCommand {
    target: Rc<RefCell<GLVertexBufferData>>,
    attributes: GLVertexAttributeDescriptorVec,
    data: Vec<u8>,
}

impl Command for CreateCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data(ll, &self.attributes, self.data.as_slice());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Rc<RefCell<GLVertexBufferData>>,
}

impl Command for ReleaseCommand {
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
        println!("GLVertexBuffer - release");

        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<Q: CommandQueue>(&mut self,
                                          queue: &mut Q,
                                          attributes: GLVertexAttributeDescriptorVec,
                                          vertex_data: &[u8]) {
        println!("GLVertexBuffer - set_copy");

        queue.add(
            CreateCommand {
                target: self.0.clone(),
                attributes: attributes,
                data: vertex_data.to_vec(),
            }
        );
    }

    pub fn get_attribute(&self, attribute_index: usize) -> GLVertexAttribute {
        assert!(attribute_index < MAX_VERTEX_ATTRIBUTE_COUNT);
        GLVertexAttribute {
            target: self.0.clone(),
            attribute_index: attribute_index,
        }
    }
}


pub type VertexBufferImpl = GLVertexBuffer;
pub type VertexAttributeDescriptorImpl = GLVertexAttributeDescriptor;
pub type VertexAttributeDescriptorImplVec = GLVertexAttributeDescriptorVec;
pub type VertexAttributeImpl = GLVertexAttribute;
pub type VertexAttributeImplVec = GLVertexAttributeVec;
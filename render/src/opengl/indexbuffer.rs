use std::rc::Rc;
use std::cell::RefCell;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::lowlevel::indexbinding::*;


/// Structure to store reference to a single attribute os a buffer
#[derive(Clone)]
pub struct GLIndexBufferRef {
    target: Rc<RefCell<GLIndexBufferData>>
}

impl GLIndexBufferRef {
    pub fn bind(&self, ll: &mut LowLevel) {
        let ib = self.target.borrow();
        ib.bind(ll);
    }
}


/// Structure to store hardware data associated to a IndexBuffer.
struct GLIndexBufferData {
    hw_id: GLuint,
    type_id: GLenum,
}

impl GLIndexBufferData {
    fn new() -> GLIndexBufferData {
        GLIndexBufferData {
            hw_id: 0,
            type_id: 0,
        }
    }

    fn upload_data(&mut self, ll: &mut LowLevel, type_id: GLenum, data: &[u8]) {
        gl_check_error();
        if self.hw_id == 0 {
            gl!(GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);
        self.type_id = type_id;

        ll.index_binding.bind_buffer(self.hw_id);
        gl!(BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       data.len() as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW));
        gl_check_error();
    }

    fn bind(&self, ll: &mut LowLevel) {
        ll.index_binding.bind_index(self.hw_id, self.type_id);
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.index_binding.unbind_if_active(self.hw_id);
        gl!(DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
        self.type_id = 0;
    }
}

impl Drop for GLIndexBufferData {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking index buffer");
    }
}


/// RenderCommand to create and allocated OpenGL resources.
struct CreateCommand {
    target: Rc<RefCell<GLIndexBufferData>>,
    type_id: GLenum,
    data: Vec<u8>,
}

impl Command for CreateCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process<'a>(&mut self, _resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data(ll, self.type_id, self.data.as_slice());
    }
}


/// RenderCommand to release the allocated OpenGL resources.
struct ReleaseCommand {
    target: Rc<RefCell<GLIndexBufferData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process<'a>(&mut self, _resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}

/// IndexBuffer implementation for OpenGL.
pub struct GLIndexBuffer(Rc<RefCell<GLIndexBufferData>>);

impl GLIndexBuffer {
    pub fn new() -> GLIndexBuffer {
        GLIndexBuffer(
            Rc::new(RefCell::new(GLIndexBufferData::new()))
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        //println!("GLIndexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<ID: IndexDeclaration, Q: CommandQueue>(&mut self, queue: &mut Q, index_data: &[u8]) {
        //println!("GLIndexBuffer - set_copy");
        queue.add(
            CreateCommand {
                target: self.0.clone(),
                type_id: ID::IndexType::get_gl_type_id(),
                data: index_data.to_vec(),
            }
        );
    }

    pub fn get_ref(&self) -> GLIndexBufferRef {
        GLIndexBufferRef {
            target: self.0.clone()
        }
    }
}

/// Trait to extract type info for indices
pub trait IndexTypeInfoImpl: GLIndexTypeInfo {}

impl<T> IndexTypeInfoImpl for T where T: GLIndexTypeInfo {}

/// The index buffer implementation
pub type IndexBufferImpl = GLIndexBuffer;

/// Reference to an index buffer
pub type IndexBufferRefImpl = GLIndexBufferRef;
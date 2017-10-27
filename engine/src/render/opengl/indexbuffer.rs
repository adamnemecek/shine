use std::rc::Rc;
use std::cell::RefCell;

use render::*;
use render::opengl::lowlevel::*;


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
            unsafe {
                gl::GenBuffers(1, &mut self.hw_id);
            }
        }
        assert!(self.hw_id != 0);
        self.type_id = type_id;

        ll.index_binding.bind_buffer(self.hw_id);
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           data.len() as GLsizeiptr,
                           data.as_ptr() as *const GLvoid,
                           gl::STATIC_DRAW);
        }
        gl_check_error();
    }

    fn bind(&self, ll: &mut LowLevel, location: GLuint, attribute: usize) {
        ll.index_binding.bind_attribute(location, self.hw_id, &self.type_id);
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.index_binding.unbind_if_active(self.hw_id);
        unsafe {
            gl::DeleteBuffers(1, &self.hw_id);
        }
        self.hw_id = 0;
        self.type_id = 0;
    }
}


/// RenderCommand to create the OpenGL program, set the shader sources and compile (link) a shader program.
struct CreateCommand {
    target: Rc<RefCell<GLIndexBufferData>>,
    type_id: GLenum,
    data: Vec<u8>,
}

impl Command for CreateCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data(ll, &self.type_id, self.data.as_slice());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Rc<RefCell<GLIndexBufferData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
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
        println!("GLIndexBuffer - release");

        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<Q: CommandQueue, D>(&mut self,
                                          queue: &mut Q,
                                          vertex_data: &[D]) {
        println!("GLIndexBuffer - set_copy");

        queue.add(
            CreateCommand {
                target: self.0.clone(),
                type_id: 0,
                data: vertex_data.to_vec(),
            }
        );
    }
}


pub type IndexBufferImpl = GLIndexBuffer;

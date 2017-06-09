use std::rc::Rc;
use std::cell::RefCell;

use render::*;

use render::opengl::lowlevel::*;
//use render::opengl::commandqueue::*;


type VertexAttributes = [VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT];

pub struct GLVertexBuffer {
    hw_id: GLuint,

    attributes: VertexAttributes,
}

impl GLVertexBuffer {
    fn new() -> GLVertexBuffer {
        GLVertexBuffer {
            hw_id: 0,
            attributes: [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT],
        }
    }

    fn upload_data(&mut self, ll: &mut LowLevel, attributes: &VertexAttributes, data: &[u8]) {
        println!("data: {:?}", data);
        gl_check_error();
        self.attributes = *attributes;
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

    fn bind<I: IntoIterator<Item=(u8, GLuint)>>(&mut self, ll: &mut LowLevel, loc_iter: I) {
        for location in loc_iter {
            ll.vertex_binding.delayed_bind_attribute(location.1, self.hw_id, &self.attributes[location.0 as usize]);
        }
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


/// Low level render command to set the vertex data
struct CreateCommand {
    target: Rc<RefCell<GLVertexBuffer>>,
    attributes: VertexAttributes,
    data: Vec<u8>,
}

impl GLCommand for CreateCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data(ll, &self.attributes, self.data.as_slice());
    }
}


/// Low level render command to release a vertex buffer
struct ReleaseCommand {
    target: Rc<RefCell<GLVertexBuffer>>,
}

impl GLCommand for ReleaseCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}


/// Structure to wrap a GLVertexBuffer into a shared resource
pub struct GLVertexBufferWrapper {
    wrapped: Rc<RefCell<GLVertexBuffer>>
}

impl GLVertexBufferWrapper {
    pub fn new() -> GLVertexBufferWrapper {
        GLVertexBufferWrapper { wrapped: Rc::new(RefCell::new(GLVertexBuffer::new())) }
    }

    pub fn wrap(wrapped: Rc<RefCell<GLVertexBuffer>>) -> GLVertexBufferWrapper {
        GLVertexBufferWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> Rc<RefCell<GLVertexBuffer>> {
        self.wrapped.clone()
    }

    pub fn set_transient<'a, VS: TransientVertexSource>(&mut self, queue: &mut CommandQueue, vertex_source: &VS) {
        println!("GLVertexBuffer - set_copy");

        let desc = vertex_source.to_vertex_source();

        queue.platform.add(
            CreateCommand {
                target: self.wrapped.clone(),
                attributes: desc.0,
                data: desc.1.to_vec(),
            }
        );
    }

    /*pub fn set_copy<'a, VD: Iterator<Item=&'a VertexAttributeImpl>>(&mut self, queue: &mut CommandQueue, vertex_descriptor: VD, vertex_data: &[u8]) {
        println!("GLVertexBuffer - set_copy");

        let mut attributes = [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT];
        for (src, dst) in attributes.iter_mut().zip(vertex_descriptor) {
            *src = *dst;
        }

        queue.platform.add(
            CreateCommand {
                target: self.wrapped.clone(),
                attributes: attributes,
                data: vertex_data.to_vec(),
            }
        );
    }*/

    pub fn release(&mut self, queue: &mut CommandQueue) {
        println!("GLVertexBuffer - release");

        queue.platform.add(
            ReleaseCommand {
                target: self.wrapped.clone()
            }
        );
    }
}

pub type VertexBufferImpl = GLVertexBufferWrapper;
pub type VertexAttributeImpl = VertexAttribute;

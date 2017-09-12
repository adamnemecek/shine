use std::rc::Rc;
use std::cell::RefCell;

use render::*;
use render::opengl::lowlevel::*;

type VertexAttributes = [VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT];


/// Structure to store hardware data associated to a VertexBuffer.
struct GLVertexBufferData {
    hw_id: GLuint,
    attributes: VertexAttributes,
}

impl GLVertexBufferData {
    fn new() -> GLVertexBufferData {
        GLVertexBufferData {
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


/// RenderCommand to create the OpenGL program, set the shader sources and compile (link) a shader program.
struct CreateCommand {
    target: Rc<RefCell<GLVertexBufferData>>,
    attributes: VertexAttributes,
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

    pub fn set_transient<'a, VS: TransientVertexSource, Q: CommandQueue>(&mut self, queue: &mut Q, vertex_source: &VS) {
        println!("GLVertexBuffer - set_copy");

        let desc = vertex_source.to_vertex_source();

        queue.add(
            CreateCommand {
                target: self.0.clone(),
                attributes: desc.0,
                data: desc.1.to_vec(),
            }
        );
    }

    /*pub fn set_copy<'a, VD: Iterator<Item=&'a VertexAttributeImpl>>(&mut self, queue: &mut GLCommandStore, vertex_descriptor: VD, vertex_data: &[u8]) {
        println!("GLVertexBuffer - set_copy");

        let mut attributes = [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT];
        for (src, dst) in attributes.iter_mut().zip(vertex_descriptor) {
            *src = *dst;
        }

        queue.add(
            CreateCommand {
                target: self.resource.clone(),
                attributes: attributes,
                data: vertex_data.to_vec(),
            }
        );
    }*/

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        println!("GLVertexBuffer - release");

        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }
}


pub type VertexBufferImpl = GLVertexBuffer;
pub type VertexAttributeImpl = VertexAttribute;

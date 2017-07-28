#![allow(dead_code)]
extern crate gl;

use std::rc::Rc;
use std::cell::RefCell;

use render::*;
use render::gl::*;


pub struct VertexBufferImpl {
    hw_id: GLuint,
}

impl VertexBufferImpl {
    fn new() -> VertexBufferImpl {
        VertexBufferImpl {
            hw_id: 0
        }
    }

    fn set(&mut self, ll: &mut LowLevel) {
        gl_check_error();
        if self.hw_id == 0 {
            unsafe {
                gl::CreateBuffers(1, &mut self.hw_id);
            }
        }

        ll.vertex_binding.bind_buffer(self.hw_id);
        //todo: update buffer
        gl_check_error();
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.vertex_binding.unbind_if_active(self.hw_id);
        unsafe {
            gl::DeleteBuffers(1, &mut self.hw_id);
        }
        self.hw_id = 0;
    }
}

impl Drop for VertexBufferImpl {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "vertex buffer was not released" );
    }
}


/*struct CreateCommand {
    program: Rc<RefCell<ShaderProgramImpl>>,
    sources: ShaderSources,
}

impl ICommand for CreateCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        println!("{:?}", self);
        self.program.borrow_mut().create_program(ll, &mut self.sources);
    }
}
*/

struct ReleaseCommand {
    vertex_buffer: Rc<RefCell<VertexBufferImpl>>,
}

impl ICommand for ReleaseCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.vertex_buffer.borrow_mut().release(ll);
    }
}


pub struct VertexBuffer(Rc<RefCell<VertexBufferImpl>>);

impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        VertexBuffer(Rc::new(RefCell::new(VertexBufferImpl::new())))
    }
}

impl IVertexBuffer for VertexBuffer {
    /*fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>>(&mut self, queue: &mut CommandQueue, sources: I)
    {
        println!("set_sources");
        queue.add(
            CreateCommand {
                program: self.0.clone(),
                sources: sources
                    .map(|&(t, s)| ShaderSource(gl_get_shader_enum(t), s.as_bytes().to_vec()))
                    .collect()
            }
        );
    }*/

    fn release(&mut self, queue: &mut CommandQueue)
    {
        queue.add(
            ReleaseCommand {
                vertex_buffer: self.0.clone()
            }
        );
    }
}

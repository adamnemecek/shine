#![allow(dead_code)]
extern crate gl;

use std::rc::Rc;
use std::cell::RefCell;
use self::gl::types::*;
use render::*;
use render::gl::utils::*;
use render::gl::lowlevel::LowLevel;
use render::gl::commandqueue::ICommand;


struct ShaderSource(GLenum, String);

type ShaderSources = Vec<ShaderSource>;


pub struct ShaderProgramImpl {
    hw_id: GLuint,
}

impl ShaderProgramImpl {
    fn new() -> ShaderProgramImpl {
        ShaderProgramImpl {
            hw_id: 0
        }
    }

    fn create_program(&mut self, ll: &mut LowLevel, sources: &Vec<ShaderSource>) {
        if self.hw_id != 0 {
            self.release(ll);
        }

        gl_check_error();
        unsafe {
            self.hw_id = gl::CreateProgram();
        }

        // compile shaders
        for &ShaderSource(shader, ref source) in sources.iter() {
            println!("shader: {}, source: {} ", shader, source);
        }

        gl_check_error();
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.program_binding.unbind_if_active(self.hw_id);
        unsafe {
            gl::DeleteProgram(self.hw_id);
        }
        self.hw_id = 0;
    }
}

impl Drop for ShaderProgramImpl {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "shader was not released" );
    }
}


struct CreateCommand {
    program: Rc<RefCell<ShaderProgramImpl>>,
    sources: ShaderSources,
}

impl ICommand for CreateCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        println!("CreateCommand::process");
        self.program.borrow_mut().create_program(ll, &mut self.sources);
    }
}


struct ReleaseCommand {
    program: Rc<RefCell<ShaderProgramImpl>>,
}

impl ICommand for ReleaseCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.program.borrow_mut().release(ll);
    }
}


pub struct ShaderProgram(Rc<RefCell<ShaderProgramImpl>>);

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram(Rc::new(RefCell::new(ShaderProgramImpl::new())))
    }
}

impl IShaderProgram for ShaderProgram {
    fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>>(&mut self, queue: &mut CommandQueue, sources: I)
    {
        println!("set_sources");
        queue.add(
            CreateCommand {
                program: self.0.clone(),
                sources: sources.map(|_| ShaderSource(0, "".to_string())).collect()
            }
        );
    }

    fn release(&mut self, queue: &mut CommandQueue)
    {
        queue.add(
            ReleaseCommand {
                program: self.0.clone()
            }
        );
    }
}

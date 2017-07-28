#![allow(dead_code)]
extern crate gl;

use std::rc::Rc;
use std::cell::RefCell;
use std::str::from_utf8;
use std::vec::Vec;

use render::*;
use render::gl::*;


struct ShaderError(String);

struct ShaderSource(GLenum, Vec<u8>);

type ShaderSources = Vec<ShaderSource>;

fn gl_get_shader_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}


pub struct ShaderProgramImpl {
    hw_id: GLuint,
}

impl ShaderProgramImpl {
    fn new() -> ShaderProgramImpl {
        ShaderProgramImpl {
            hw_id: 0
        }
    }

    fn attach_shader(&mut self, shader_type: GLenum, shader_source: &Vec<u8>) -> Result<(), ShaderError> {
        gl_check_error();
        unsafe {
            let shader_id = gl::CreateShader(shader_type);
            let source_len = shader_source.len() as GLint;
            let sources_ptr = shader_source.as_ptr() as *const GLchar;
            gl::ShaderSource(shader_id, 1, &sources_ptr as *const *const GLchar, &source_len as *const GLsizei);
            gl::CompileShader(shader_id);

            let mut status: GLint = gl::FALSE as GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);

            if status != gl::TRUE as GLint {
                let mut info_len: GLsizei = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_len);
                let info_buf = vec![0u8; info_len as usize];
                gl::GetShaderInfoLog(shader_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar);
                let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
                gl::DeleteShader(shader_id);
                gl_check_error();
                Err(ShaderError(result_msg))
            } else {
                gl::AttachShader(self.hw_id, shader_id);
                gl::DeleteShader(shader_id);
                gl_check_error();
                Ok(())
            }
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

        // create and attach shaders
        gl_check_error();
        for &ShaderSource(shader, ref source) in sources.iter() {
            //println!("compiling shader:\n{}", from_utf8(source.as_slice()).unwrap());
            let shader_res = self.attach_shader(shader, &source);
            if let Some(ShaderError(msg)) = shader_res.err() {
                println!("shader compilation failed.\nsource:\n{}\nerror:\n{}", from_utf8(source.as_slice()).unwrap(), msg);
                self.release(ll);
                return
            }
        }

        gl_check_error();
        unsafe {
            gl::LinkProgram(self.hw_id);

            let mut status: GLint = gl::FALSE as GLint;
            gl::GetProgramiv(self.hw_id, gl::LINK_STATUS, &mut status);

            if status != gl::TRUE as GLint {
                // link failed, find error message
                let mut info_len: GLsizei = 0;
                gl::GetProgramiv(self.hw_id, gl::INFO_LOG_LENGTH, &mut info_len);
                let info_buf = vec![0u8; info_len as usize];
                gl::GetProgramInfoLog(self.hw_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar);
                let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
                println!("program link failed:\n{}", result_msg);
                self.release(ll);
                return;
            }
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
        assert! ( self.hw_id == 0, "shader was not released" );
    }
}


struct CreateCommand {
    program: Rc<RefCell<ShaderProgramImpl>>,
    sources: ShaderSources,
}

impl ICommand for CreateCommand {
    fn process(&mut self, ll: &mut LowLevel) {
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
                sources: sources
                    .map(|&(t, s)| ShaderSource(gl_get_shader_enum(t), s.as_bytes().to_vec()))
                    .collect()
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

#![allow(dead_code)]
extern crate gl;

use self::gl::types::*;
//use super::device::Window;
use super::lowlevel::LowLevel;
use super::utils::{gl_check_error};


pub struct ShaderProgram {
    hw_id: GLuint,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            hw_id: 0
        }
    }

    pub fn create_program<'a, I: Iterator<Item=&'a (GLenum, &'a str)>>(&mut self, ll: &mut LowLevel, sources: I) {
        if self.hw_id != 0 {
            self.release(ll);
        }

        gl_check_error();
        unsafe {
            self.hw_id = gl::CreateProgram();
        }

        // compile shaders
        for &(shader, source) in sources {
            println!("shader: {}, source: {} ", shader, source);
        }

        gl_check_error();
        println!("program created: {}", self.hw_id);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
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

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "shader was not released" );
    }
}
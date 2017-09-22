#![allow(dead_code)]

use render::opengl::lowlevel::*;

pub struct ProgramBinding {
    force: bool,
    bound_id: GLuint,
}

impl ProgramBinding {
    pub fn new() -> ProgramBinding {
        ProgramBinding {
            force: false,
            bound_id: 0,
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_forced(&mut self, force: bool) {
        self.force = force;
    }

    /// Returns the id of the current program.
    pub fn get_bound_id(&self) -> GLuint {
        self.bound_id
    }

    /// Binds program.
    pub fn bind(&mut self, program_id: GLuint) {
        if !self.force && self.bound_id == program_id {
            return;
        }

        gl_check_error();
        unsafe {
            gl::UseProgram(program_id);
        }
        gl_check_error();
        self.bound_id = program_id;
    }

    /// Unbinds the program if it is active. This function is usualy used during release.
    pub fn unbind_if_active(&mut self, program_id: GLuint) {
        if self.bound_id == program_id {
            self.bind(0);
        }
    }
}

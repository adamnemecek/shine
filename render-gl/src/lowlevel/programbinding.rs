#![allow(dead_code)]

use lowlevel::*;


/// Handle shader binding states
pub struct ProgramBinding {
    force: bool,
    bound_id: GLuint,
    params: Option<ProgramParameters>,
}

impl ProgramBinding {
    pub fn new() -> ProgramBinding {
        ProgramBinding {
            force: false,
            bound_id: 0,
            params: None,
        }
    }

    /// Converts a ShaderType enum to the corresponding GLenum.
    pub fn glenum_from_shader_type(shader_type: ShaderType) -> GLenum {
        match shader_type {
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER
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

    /// Returns the id of the current program.
    pub fn get_parameters(&self) -> Option<ProgramParameters> {
        self.params.clone()
    }

    /// Binds program.
    pub fn bind(&mut self, program_id: GLuint, params: Option<ProgramParameters>) {
        if !self.force && self.bound_id == program_id {
            return;
        }

        gl_check_error();
        ffi!(gl::UseProgram(program_id));
        gl_check_error();
        self.bound_id = program_id;
        self.params = params;
    }

    /// Unbinds the program if it is active. This function is mainly used during release.
    pub fn unbind_if_active(&mut self, program_id: GLuint) {
        if self.bound_id == program_id {
            self.bind(0, None);
        }
    }

    pub fn commit(&mut self) {}
}

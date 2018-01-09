#![allow(dead_code)]

use std::str::from_utf8;
use arrayvec::ArrayVec;
use lowlevel::*;
use limits::*;


/// Error reported by the driver during compilation and linking
#[derive(Clone, Debug)]
struct ShaderError(String);


/// Shader parameters info: required type, binding locations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParameterLocation {
    Attribute {
        location: GLuint,
        size: GLint,
        type_id: GLenum,
    },

    Uniform {
        location: GLuint,
        size: GLint,
        type_id: GLenum,
    },

    Empty,
}

impl ParameterLocation {
    fn is_valid(&self) -> bool {
        match self {
            &ParameterLocation::Attribute { type_id, .. } => type_id != 0,
            &ParameterLocation::Uniform { type_id, .. } => type_id != 0,
            _ => false
        }
    }
}


/// Attributes in the order defined by the descriptor
pub type ParameterLocations = ArrayVec<[ParameterLocation; MAX_USED_PARAMETER_COUNT]>;


/// Structure to store hardware data associated to a ShaderProgram.
pub struct GLShaderProgram {
    hw_id: GLuint,
    parameter_locations: ParameterLocations,
}

impl GLShaderProgram {
    pub fn new() -> GLShaderProgram {
        GLShaderProgram {
            hw_id: 0,
            parameter_locations: ParameterLocations::new(),
        }
    }

    fn attach_shader(&mut self, shader_type: GLenum, shader_source: &[u8]) -> Result<(), ShaderError> {
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

    pub fn create_program<DECL: ShaderDeclaration>(&mut self, ll: &mut LowLevel) {
        gl_check_error();
        if self.hw_id != 0 {
            self.release(ll);
        }

        gl_check_error();
        self.hw_id = gl!(CreateProgram());

        // create and attach shaders
        gl_check_error();
        for source in DECL::get_sources() {
            if let Err(ShaderError(err)) = self.attach_shader(ProgramBinding::glenum_from_shader_type(source.0), source.1.as_bytes()) {
                println!("Shader program compilation failed.\n{}\nError:{}", source.1, err);
                self.release(ll);
                return;
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
                println!("Shader program link failed:\n{}", result_msg);
                self.release(ll);
                return;
            }
        }

        gl_check_error();
    }

    fn parse_attributes<DECL: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut attribute_size: GLint = 0;
        let mut attribute_type: GLenum = 0;

        gl_check_error();
        gl!(GetProgramiv(self.hw_id, gl::ACTIVE_ATTRIBUTES, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_ATTRIBUTE_COUNT, "Too many vertex attributes in the shader. Allowed count {} but {} was found.", MAX_USED_ATTRIBUTE_COUNT, count);

        for location in 0..count {
            gl_check_error();
            gl!(GetActiveAttrib(self.hw_id,
                                location,
                                name_buffer.len() as GLint,
                                &mut name_length,
                                &mut attribute_size,
                                &mut attribute_type,
                                name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let attribute_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = DECL::Parameters::get_index_by_name(&attribute_name).expect(&format!("Vertex attribute name {} could not be resolved", attribute_name));
            let attribute = &mut self.parameter_locations[param_idx];

            assert!(*attribute == ParameterLocation::Empty);
            *attribute = ParameterLocation::Attribute {
                location: location,
                size: attribute_size,
                type_id: attribute_type,
            };
            //println!("Shader program attribute {}({})= {:?}", attribute_name, attribute_idx, attribute);
        }
    }

    fn parse_uniforms<DECL: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut uniform_size: GLint = 0;
        let mut uniform_type: GLenum = 0;

        gl_check_error();
        gl!(GetProgramiv(self.hw_id, gl::ACTIVE_UNIFORMS, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_UNIFORM_COUNT, "Too many uniforms in the shader: {}/{}", count, MAX_USED_UNIFORM_COUNT);

        for location in 0..count {
            gl_check_error();
            gl!(GetActiveUniform(self.hw_id,
                                 location,
                                 name_buffer.len() as GLint,
                                 &mut name_length,
                                 &mut uniform_size,
                                 &mut uniform_type,
                                 name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let uniform_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = DECL::Parameters::get_index_by_name(&uniform_name).expect(&format!("Uniform name {} could not be resolved", uniform_name));
            let uniform = &mut self.parameter_locations[param_idx];

            assert!(*uniform == ParameterLocation::Empty);
            *uniform = ParameterLocation::Uniform {
                location: location,
                size: uniform_size,
                type_id: uniform_type,
            };

            //println!("Shader program uniform {}({})= {:?}", uniform_name, uniform_idx, uniform);
        }
    }

    pub fn parse_parameters<DECL: ShaderDeclaration>(&mut self, ll: &mut LowLevel) {
        (0..DECL::Parameters::get_count()).for_each(|_| self.parameter_locations.push(ParameterLocation::Empty));
        assert!(self.parameter_locations.len() <= MAX_USED_PARAMETER_COUNT, "Too many shader parameters in declaration, allowed count: {}", MAX_USED_PARAMETER_COUNT);

        self.parse_attributes::<DECL>(ll);
        self.parse_uniforms::<DECL>(ll);

        println!("shader parameters: {:?}", self.parameter_locations);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        gl_check_error();
        ll.program_binding.unbind_if_active(self.hw_id);
        gl_check_error();
        gl!(DeleteProgram(self.hw_id));
        gl_check_error();

        self.hw_id = 0;
        self.parameter_locations.clear();
    }

    fn draw<F: Fn(&mut LowLevel, &ParameterLocations)>(&mut self, ll: &mut LowLevel, parameter_update: &F,
                                                       primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
        // bind shader
        if self.hw_id == 0 {
            // no drawing when shader is not valid
            return;
        }
        ll.program_binding.bind(self.hw_id);

        // bind parameters
        gl_check_error();
        parameter_update(ll, &self.parameter_locations);
        gl_check_error();

        ll.draw(primitive, vertex_start, vertex_count);
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "Leaking shader program");
    }
}

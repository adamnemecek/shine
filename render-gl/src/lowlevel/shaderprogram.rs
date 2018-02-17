#![allow(dead_code)]

use std::str::from_utf8;
use lowlevel::*;
use libconfig::*;


/// Error reported by the driver during compilation and linking
#[derive(Clone, Debug)]
struct ShaderError(String);


/// Structure to store hardware data associated to a ShaderProgram.
pub struct GLShaderProgram {
    hw_id: GLuint,
    parameter_locations: ProgramParameters,
}

impl GLShaderProgram {
    pub fn new() -> GLShaderProgram {
        GLShaderProgram {
            hw_id: 0,
            parameter_locations: Default::default(),
        }
    }

    fn attach_shader(&mut self, shader_type: GLenum, shader_source: &[u8]) -> Result<(), ShaderError> {
        gl_check_error();

        let shader_id = ffi!(gl::CreateShader(shader_type));
        let source_len = shader_source.len() as GLint;
        let sources_ptr = shader_source.as_ptr() as *const GLchar;
        ffi!(gl::ShaderSource(shader_id, 1, &sources_ptr as *const *const GLchar, &source_len as *const GLsizei));
        ffi!(gl::CompileShader(shader_id));

        let mut status: GLint = gl::FALSE as GLint;
        ffi!(gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status));

        if status != gl::TRUE as GLint {
            let mut info_len: GLsizei = 0;
            ffi!(gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_len));
            let info_buf = vec![0u8; info_len as usize];
            ffi!(gl::GetShaderInfoLog(shader_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar));
            let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
            ffi!(gl::DeleteShader(shader_id));
            gl_check_error();
            Err(ShaderError(result_msg))
        } else {
            ffi!(gl::AttachShader(self.hw_id, shader_id));
            ffi!(gl::DeleteShader(shader_id));
            gl_check_error();
            Ok(())
        }
    }

    pub fn create_program<'a, I: Iterator<Item=&'a (ShaderType, &'static str)>>(&mut self, ll: &mut LowLevel, sources: I) {
        gl_check_error();
        if self.hw_id != 0 {
            self.release(ll);
        }

        gl_check_error();
        self.hw_id = ffi!(gl::CreateProgram());

        // create and attach shaders
        gl_check_error();
        for source in sources {
            if let Err(ShaderError(err)) = self.attach_shader(ProgramBinding::glenum_from_shader_type(source.0), source.1.as_bytes()) {
                println!("Shader program compilation failed.\n{}\nError:{}", source.1, err);
                self.release(ll);
                return;
            }
        }

        gl_check_error();
        ffi!(gl::LinkProgram(self.hw_id));

        let mut status: GLint = gl::FALSE as GLint;
        ffi!(gl::GetProgramiv(self.hw_id, gl::LINK_STATUS, &mut status));

        if status != gl::TRUE as GLint {
            let mut info_len: GLsizei = 0;
            ffi!(gl::GetProgramiv(self.hw_id, gl::INFO_LOG_LENGTH, &mut info_len));
            let info_buf = vec![0u8; info_len as usize];
            ffi!(gl::GetProgramInfoLog(self.hw_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar));
            let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
            println!("Shader program link failed:\n{}", result_msg);
            self.release(ll);
            return;
        }

        gl_check_error();
    }

    fn parse_index<F: Fn(&str) -> Option<usize>>(&mut self, _ll: &mut LowLevel, name_to_index: &F) {
        let parameter_locations = &mut *self.parameter_locations.borrow_mut();

        let param_idx = name_to_index("indices").expect(&format!("Unknown shader (index) parameter 'indices'"));
        assert!(param_idx <= MAX_USED_PARAMETER_COUNT, "Shader (index) parameters index is out of bounds, allowed: {}, but got {} for 'indices'", MAX_USED_PARAMETER_COUNT, param_idx);
        let attribute = &mut parameter_locations[param_idx];

        assert!(*attribute == ProgramParameter::Empty);
        *attribute = ProgramParameter::Index;
    }

    fn parse_attributes<F: Fn(&str) -> Option<usize>>(&mut self, _ll: &mut LowLevel, name_to_index: &F) {
        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut attribute_size: GLint = 0;
        let mut attribute_type: GLenum = 0;

        gl_check_error();
        ffi!(gl::GetProgramiv(self.hw_id, gl::ACTIVE_ATTRIBUTES, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_ATTRIBUTE_COUNT, "Too many vertex attributes in the shader. Allowed count {} but {} was found.", MAX_USED_ATTRIBUTE_COUNT, count);

        let parameter_locations = &mut *self.parameter_locations.borrow_mut();

        for location in 0..count {
            gl_check_error();
            ffi!(gl::GetActiveAttrib(self.hw_id,
                                location,
                                name_buffer.len() as GLint,
                                &mut name_length,
                                &mut attribute_size,
                                &mut attribute_type,
                                name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let attribute_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = name_to_index(&attribute_name).expect(&format!("Unknown shader (attribute) parameter '{}'", attribute_name));
            assert!(param_idx <= MAX_USED_PARAMETER_COUNT, "Shader (attribute) parameters index is out of bounds, allowed: {}, but got {} for '{}'", MAX_USED_PARAMETER_COUNT, param_idx, attribute_name);
            let attribute = &mut parameter_locations[param_idx];

            assert!(*attribute == ProgramParameter::Empty);
            *attribute = ProgramParameter::Attribute {
                name: attribute_name,
                location: location,
                size: attribute_size,
                type_id: attribute_type,
            };
            //println!("Shader program attribute {}({})= {:?}", attribute_name, param_idx, attribute);
        }
    }

    fn parse_uniforms<F: Fn(&str) -> Option<usize>>(&mut self, _ll: &mut LowLevel, name_to_index: &F) {
        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut uniform_size: GLint = 0;
        let mut uniform_type: GLenum = 0;

        gl_check_error();
        ffi!(gl::GetProgramiv(self.hw_id, gl::ACTIVE_UNIFORMS, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_UNIFORM_COUNT, "Too many uniforms in the shader: {}/{}", count, MAX_USED_UNIFORM_COUNT);

        let parameter_locations = &mut *self.parameter_locations.borrow_mut();

        for location in 0..count {
            gl_check_error();
            ffi!(gl::GetActiveUniform(self.hw_id,
                                 location,
                                 name_buffer.len() as GLint,
                                 &mut name_length,
                                 &mut uniform_size,
                                 &mut uniform_type,
                                 name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let uniform_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = name_to_index(&uniform_name).expect(&format!("Unknown shader (uniform) parameter '{}'", uniform_name));
            assert!(param_idx <= MAX_USED_PARAMETER_COUNT, "Shader (uniform) parameters index is out of bounds, allowed: {}, but got {} for '{}'", MAX_USED_PARAMETER_COUNT, param_idx, uniform_name);
            let uniform = &mut parameter_locations[param_idx];

            assert!(*uniform == ProgramParameter::Empty);
            *uniform = ProgramParameter::Uniform {
                name: uniform_name,
                location: location,
                size: uniform_size,
                type_id: uniform_type,
            };
            //println!("Shader program uniform {}({})= {:?}", uniform_name, param_idx, uniform);
        }
    }

    pub fn parse_parameters<F: Fn(&str) -> Option<usize>>(&mut self, ll: &mut LowLevel, name_to_index: F) {
        self.parameter_locations = Default::default();

        self.parse_index(ll, &name_to_index);
        self.parse_attributes(ll, &name_to_index);
        self.parse_uniforms(ll, &name_to_index);
        println!("shader parameters: {:?}", self.parameter_locations);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        gl_check_error();
        ll.program_binding.unbind_if_active(self.hw_id);
        gl_check_error();
        ffi!(gl::DeleteProgram(self.hw_id));
        gl_check_error();

        self.hw_id = 0;
        self.parameter_locations = Default::default();
    }

    pub fn get_parameter_locations(&self) -> &ProgramParameters {
        &self.parameter_locations
    }

    pub fn bind(&self, ll: &mut LowLevel) -> bool {
        if self.hw_id == 0 {
            // no drawing when shader is not valid
            return false;
        }

        ll.program_binding.bind(self.hw_id, Some(self.parameter_locations.clone()));
        true
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking shader program");
    }
}

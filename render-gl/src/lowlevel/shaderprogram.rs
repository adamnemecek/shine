#![allow(dead_code)]

use std::mem;
use std::str::from_utf8;
use lowlevel::*;
use libconfig::*;


/// Error reported by the driver during compilation and linking
#[derive(Clone, Debug)]
struct ShaderError(String);


/// Shader parameters info: required type, binding locations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShaderParameterLocation {
    Attribute {
        name: String,
        location: GLuint,
        size: GLint,
        type_id: GLenum,
    },

    Uniform {
        name: String,
        location: GLuint,
        size: GLint,
        type_id: GLenum,
    },

    Empty,
}

impl ShaderParameterLocation {
    pub fn is_valid(&self) -> bool {
        match self {
            &ShaderParameterLocation::Attribute { type_id, .. } => type_id != 0,
            &ShaderParameterLocation::Uniform { type_id, .. } => type_id != 0,
            _ => false
        }
    }

    pub fn set_attribute<A: Into<usize>>(&self, ll: &mut LowLevel, vb: &GLVertexBuffer, attribute: A) {
        match self {
            &ShaderParameterLocation::Attribute { location, type_id, .. } =>
                if type_id != 0 {
                    gl_check_error();
                    vb.bind(ll, location, attribute.into());
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32(&self, _ll: &mut LowLevel, data: f32) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT && size == 1);
                    gl_check_error();
                    ugl!(Uniform1fv(location as i32, size, mem::transmute(&data)));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x2(&self, _ll: &mut LowLevel, data: &Float32x2) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC2 && size == 1);
                    gl_check_error();
                    ugl!(Uniform2fv(location as i32, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x3(&self, _ll: &mut LowLevel, data: &Float32x3) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC3 && size == 1);
                    gl_check_error();
                    ugl!(Uniform3fv(location as i32, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x4(&self, _ll: &mut LowLevel, data: &Float32x4) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC4 && size == 1);
                    gl_check_error();
                    ugl!(Uniform4fv(location as i32, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x16(&self, _ll: &mut LowLevel, data: &Float32x16) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_MAT4 && size == 1);
                    gl_check_error();
                    ugl!(UniformMatrix4fv(location as i32, size, gl::FALSE, mem::transmute(data)));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }


    pub fn set_texture(&self, ll: &mut LowLevel, tx: &GLTexture) {
        match self {
            &ShaderParameterLocation::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    // todo: check if texture conforms to the sampler
                    assert!(type_id == gl::SAMPLER_2D && size == 1);
                    gl_check_error();
                    let slot = tx.bind(ll) as u32;
                    ugl!(Uniform1i(location as i32, slot as i32));
                    gl_check_error();
                },
            &ShaderParameterLocation::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }
}

impl Default for ShaderParameterLocation {
    fn default() -> ShaderParameterLocation {
        ShaderParameterLocation::Empty
    }
}


/// Type to store the required shader parameter locations.
pub type ParameterLocations = [ShaderParameterLocation; MAX_USED_PARAMETER_COUNT];


/// Structure to store hardware data associated to a ShaderProgram.
pub struct GLShaderProgram {
    hw_id: GLuint,
    parameter_locations: ParameterLocations,
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

        let shader_id = ugl!(CreateShader(shader_type));
        let source_len = shader_source.len() as GLint;
        let sources_ptr = shader_source.as_ptr() as *const GLchar;
        ugl!(ShaderSource(shader_id, 1, &sources_ptr as *const *const GLchar, &source_len as *const GLsizei));
        ugl!(CompileShader(shader_id));

        let mut status: GLint = gl::FALSE as GLint;
        ugl!(GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status));

        if status != gl::TRUE as GLint {
            let mut info_len: GLsizei = 0;
            ugl!(GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_len));
            let info_buf = vec![0u8; info_len as usize];
            ugl!(GetShaderInfoLog(shader_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar));
            let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
            ugl!(DeleteShader(shader_id));
            gl_check_error();
            Err(ShaderError(result_msg))
        } else {
            ugl!(AttachShader(self.hw_id, shader_id));
            ugl!(DeleteShader(shader_id));
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
        self.hw_id = ugl!(CreateProgram());

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
        ugl!(LinkProgram(self.hw_id));

        let mut status: GLint = gl::FALSE as GLint;
        ugl!(GetProgramiv(self.hw_id, gl::LINK_STATUS, &mut status));

        if status != gl::TRUE as GLint {
            let mut info_len: GLsizei = 0;
            ugl!(GetProgramiv(self.hw_id, gl::INFO_LOG_LENGTH, &mut info_len));
            let info_buf = vec![0u8; info_len as usize];
            ugl!(GetProgramInfoLog(self.hw_id, info_len, &mut info_len, info_buf.as_ptr() as *mut GLchar));
            let result_msg = from_utf8(info_buf.as_slice()).unwrap().to_string();
            println!("Shader program link failed:\n{}", result_msg);
            self.release(ll);
            return;
        }

        gl_check_error();
    }

    fn parse_attributes<F: Fn(&str) -> Option<usize>>(&mut self, _ll: &mut LowLevel, name_to_index: &F) {
        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut attribute_size: GLint = 0;
        let mut attribute_type: GLenum = 0;

        gl_check_error();
        ugl!(GetProgramiv(self.hw_id, gl::ACTIVE_ATTRIBUTES, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_ATTRIBUTE_COUNT, "Too many vertex attributes in the shader. Allowed count {} but {} was found.", MAX_USED_ATTRIBUTE_COUNT, count);

        for location in 0..count {
            gl_check_error();
            ugl!(GetActiveAttrib(self.hw_id,
                                location,
                                name_buffer.len() as GLint,
                                &mut name_length,
                                &mut attribute_size,
                                &mut attribute_type,
                                name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let attribute_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = name_to_index(&attribute_name).expect(&format!("Unknown shader (attribute) parameter name: {}", attribute_name));
            assert!(param_idx <= MAX_USED_PARAMETER_COUNT, "Shader (attribute) parameters index is out of bounds, allowed: {}, but got {} for {}", MAX_USED_PARAMETER_COUNT, param_idx, attribute_name);
            let attribute = &mut self.parameter_locations[param_idx];

            assert!(*attribute == ShaderParameterLocation::Empty);
            *attribute = ShaderParameterLocation::Attribute {
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
        ugl!(GetProgramiv(self.hw_id, gl::ACTIVE_UNIFORMS, &mut count));
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_UNIFORM_COUNT, "Too many uniforms in the shader: {}/{}", count, MAX_USED_UNIFORM_COUNT);

        for location in 0..count {
            gl_check_error();
            ugl!(GetActiveUniform(self.hw_id,
                                 location,
                                 name_buffer.len() as GLint,
                                 &mut name_length,
                                 &mut uniform_size,
                                 &mut uniform_type,
                                 name_buffer.as_ptr() as *mut GLchar));
            gl_check_error();

            let uniform_name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let param_idx = name_to_index(&uniform_name).expect(&format!("Unknown shader (uniform) parameter name: {}", uniform_name));
            assert!(param_idx <= MAX_USED_PARAMETER_COUNT, "Shader (uniform) parameters index is out of bounds, allowed: {}, but got {} for {}", MAX_USED_PARAMETER_COUNT, param_idx, uniform_name);
            let uniform = &mut self.parameter_locations[param_idx];

            assert!(*uniform == ShaderParameterLocation::Empty);
            *uniform = ShaderParameterLocation::Uniform {
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

        self.parse_attributes(ll, &name_to_index);
        self.parse_uniforms(ll, &name_to_index);
        //println!("shader parameters: {:?}", self.parameter_locations);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        gl_check_error();
        ll.program_binding.unbind_if_active(self.hw_id);
        gl_check_error();
        ugl!(DeleteProgram(self.hw_id));
        gl_check_error();

        self.hw_id = 0;
        self.parameter_locations = Default::default();
    }

    pub fn draw<F: Fn(&mut LowLevel, &ParameterLocations)>(&mut self, ll: &mut LowLevel,
                                                           primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint,
                                                           parameter_update: F) {
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
        assert!(self.hw_id == 0, "Leaking shader program");
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::str::from_utf8;
use std::marker::PhantomData;
use std::mem;

use arrayvec::ArrayVec;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::commandqueue::*;


#[derive(Clone, Debug)]
struct ShaderError(String);

/// Converts a ShaderType enum to the corresponding GLenum.
fn gl_get_shader_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}


/// Attribute info
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct AttributeLocation {
    location: GLuint,
    size: GLint,
    type_id: GLenum,
}

impl AttributeLocation {
    fn is_valid(&self) -> bool {
        self.type_id != 0
    }
}


/// Uniform info
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct UniformLocation {
    location: GLuint,
    size: GLint,
    type_id: GLenum,
}

impl UniformLocation {
    fn is_valid(&self) -> bool {
        self.type_id != 0
    }
}


/// Union type for shader parameters: attributes, uniforms
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ParameterLocation {
    Attribute(AttributeLocation),
    Uniform(UniformLocation),
    Empty,
}


/// Attributes in the order defined by the descriptor
type ParameterLocations = ArrayVec<[ParameterLocation; MAX_USED_PARAMETER_COUNT]>;


/// Helper to upload shader parameters
struct ParameterUploader<'a> {
    locations: &'a ParameterLocations,
    ll: &'a mut LowLevel
}

impl<'a> ShaderParameterVisitor for ParameterUploader<'a> {
    fn process_f32x16(&mut self, idx: usize, data: &Float32x16) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::FLOAT_MAT4 && loc.size == 1);
                gl_check_error();
                gl!(UniformMatrix4fv(loc.location as i32, loc.size, gl::FALSE, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x4(&mut self, idx: usize, data: &Float32x4) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::FLOAT_VEC4 && loc.size == 1);
                gl_check_error();
                gl!(Uniform4fv(loc.location as i32, loc.size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x3(&mut self, idx: usize, data: &Float32x3) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::FLOAT_VEC3 && loc.size == 1);
                gl_check_error();
                gl!(Uniform3fv(loc.location as i32, loc.size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x2(&mut self, idx: usize, data: &Float32x2) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::FLOAT_VEC2 && loc.size == 1);
                gl_check_error();
                gl!(Uniform2fv(loc.location as i32, loc.size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32(&mut self, idx: usize, data: f32) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::FLOAT && loc.size == 1);
                gl_check_error();
                gl!(Uniform1fv(loc.location as i32, loc.size, mem::transmute(&data)));
                gl_check_error();
            }
        }
    }

    fn process_tex_2d(&mut self, idx: usize, data: &Texture2DRefImpl) {
        if let ParameterLocation::Uniform(loc) = self.locations[idx] {
            if loc.is_valid() {
                assert!(loc.type_id == gl::SAMPLER_2D && loc.size == 1);
                gl_check_error();
                let slot = data.bind(self.ll);
                let slot = slot as u32;
                gl!(Uniform1i(loc.location as i32, slot as i32));
                gl_check_error();
            }
        }
    }

    fn process_attribute(&mut self, idx: usize, data: &VertexAttributeRefImpl) {
        if let ParameterLocation::Attribute(loc) = self.locations[idx] {
            if loc.is_valid() {
                gl_check_error();
                data.bind(self.ll, loc.location);
                gl_check_error();
            }
        }
    }
}


/// Structure to store hardware data associated to a ShaderProgram.
struct GLShaderProgramData {
    hw_id: GLuint,
    parameter_locations: ParameterLocations,
}

impl GLShaderProgramData {
    fn new() -> GLShaderProgramData {
        GLShaderProgramData {
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

    fn create_program<SD: ShaderDeclaration>(&mut self, ll: &mut LowLevel) {
        gl_check_error();
        if self.hw_id != 0 {
            self.release(ll);
        }

        gl_check_error();
        self.hw_id = gl!(CreateProgram());

        // create and attach shaders
        gl_check_error();
        for source in SD::get_sources() {
            if let Err(ShaderError(err)) = self.attach_shader(gl_get_shader_enum(source.0), source.1.as_bytes()) {
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

    fn parse_parameters<SD: ShaderDeclaration>(&mut self, ll: &mut LowLevel) {
        (0..SD::Parameters::get_count()).for_each(|_| self.parameter_locations.push(ParameterLocation::Empty));
        assert!(self.parameter_locations.len() <= MAX_USED_PARAMETER_COUNT, "Too many shader parameters in declaration, allowed count: {}", MAX_USED_PARAMETER_COUNT);

        self.parse_attributes::<SD>(ll);
        self.parse_uniforms::<SD>(ll);

        println!("shader parameters: {:?}", self.parameter_locations);
    }

    fn parse_attributes<SD: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
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
            let param_idx = SD::Parameters::get_index_by_name(&attribute_name).expect(&format!("Vertex attribute name {} could not be resolved", attribute_name));
            let attribute = &mut self.parameter_locations[param_idx];

            assert!(*attribute == ParameterLocation::Empty);
            *attribute = ParameterLocation::Attribute(AttributeLocation {
                location: location,
                size: attribute_size,
                type_id: attribute_type,
            });
            //println!("Shader program attribute {}({})= {:?}", attribute_name, attribute_idx, attribute);
        }
    }

    fn parse_uniforms<SD: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
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
            let param_idx = SD::Parameters::get_index_by_name(&uniform_name).expect(&format!("Uniform name {} could not be resolved", uniform_name));
            let uniform = &mut self.parameter_locations[param_idx];

            assert!(*uniform == ParameterLocation::Empty);
            *uniform = ParameterLocation::Uniform(UniformLocation {
                location: location,
                size: uniform_size,
                type_id: uniform_type,
            });

            //println!("Shader program uniform {}({})= {:?}", uniform_name, uniform_idx, uniform);
        }
    }

    fn release(&mut self, ll: &mut LowLevel) {
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

    fn draw<P: ShaderParameters>(&mut self, ll: &mut LowLevel,
                                 parameters: &P, indices: Option<&GLIndexBufferRef>,
                                 primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
        // bind shader
        if self.hw_id == 0 {
            // no drawing when shader is not valid
            return;
        }
        ll.program_binding.bind(self.hw_id);

        // bind parameters
        gl_check_error();
        parameters.visit(&mut ParameterUploader {
            locations: &self.parameter_locations,
            ll: ll
        });
        gl_check_error();

        // bind indices
        if let Some(ref ib) = indices {
            ib.bind(ll);
        } else {
            ll.index_binding.bind_no_index();
        }

        ll.draw(primitive, vertex_start, vertex_count);
    }
}

impl Drop for GLShaderProgramData {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "Leaking shader program");
    }
}


/// RenderCommand to allocate the OpenGL program, set the shader sources and compile (link) a shader program
struct CreateCommand<SD: ShaderDeclaration> {
    target: Rc<RefCell<GLShaderProgramData>>,
    phantom_sd: PhantomData<SD>,
}

impl<SD: ShaderDeclaration> Command for CreateCommand<SD> {
    fn get_sort_key(&self) -> usize {
        1
    }

    fn process(&mut self, ll: &mut LowLevel) {
        let ref mut shader = *self.target.borrow_mut();
        shader.create_program::<SD>(ll);
        shader.parse_parameters::<SD>(ll);
    }
}


/// RenderCommand to release the allocated OpenGL program.
struct ReleaseCommand {
    target: Rc<RefCell<GLShaderProgramData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        1
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}


/// RenderCommand to submit a geometry for rendering
struct DrawCommand<SD: ShaderDeclaration> {
    target: Rc<RefCell<GLShaderProgramData>>,
    parameters: SD::Parameters,
    indices: Option<GLIndexBufferRef>,
    primitive: GLenum,
    vertex_start: GLuint,
    vertex_count: GLuint,
}

impl<SD: ShaderDeclaration> Command for DrawCommand<SD> {
    fn get_sort_key(&self) -> usize {
        1
    }

    fn process(&mut self, ll: &mut LowLevel) {
        let ref mut shader = *self.target.borrow_mut();
        shader.draw(ll, &self.parameters, self.indices.as_ref(), self.primitive, self.vertex_start, self.vertex_count);
    }
}


/// ShaderProgram implementation for OpenGL.
pub struct GLShaderProgram(Rc<RefCell<GLShaderProgramData>>);

impl GLShaderProgram {
    pub fn new() -> GLShaderProgram {
        GLShaderProgram(
            Rc::new(RefCell::new(GLShaderProgramData::new()))
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn compile<SD: ShaderDeclaration, Q: CommandQueue>(&mut self, queue: &mut Q) {
        queue.add(
            CreateCommand::<SD> {
                target: self.0.clone(),
                phantom_sd: PhantomData,
            }
        );
    }

    pub fn draw<SD: ShaderDeclaration, Q: CommandQueue>(&mut self, queue: &mut Q,
                                                        parameters: SD::Parameters,
                                                        primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        queue.add(
            DrawCommand::<SD> {
                target: self.0.clone(),
                parameters: parameters,
                indices: None,
                primitive: gl_get_primitive_enum(primitive),
                vertex_start: vertex_start as GLuint,
                vertex_count: vertex_count as GLuint,
            }
        );
    }

    pub fn draw_indexed<SD: ShaderDeclaration, Q: CommandQueue>(&mut self, queue: &mut Q,
                                                                parameters: SD::Parameters,
                                                                indices: IndexBufferRefImpl,
                                                                primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        queue.add(
            DrawCommand::<SD> {
                target: self.0.clone(),
                parameters: parameters,
                indices: Some(indices),
                primitive: gl_get_primitive_enum(primitive),
                vertex_start: vertex_start as GLuint,
                vertex_count: vertex_count as GLuint,
            }
        );
    }
}

/// The shader program implementation
pub type ShaderProgramImpl = GLShaderProgram;

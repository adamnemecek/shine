#![allow(dead_code)]

use std::str::from_utf8;
use std::marker::PhantomData;
use std::mem;

use arrayvec::ArrayVec;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::commandqueue::*;
use store::handlestore::*;


/// Error reported by the driver during compilation and linking
#[derive(Clone, Debug)]
struct ShaderError(String);


/// Converts a ShaderType enum to the corresponding GLenum.
fn gl_get_shader_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}


/// Shader parameters info: required type, binding locations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ParameterLocation {
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
type ParameterLocations = ArrayVec<[ParameterLocation; MAX_USED_PARAMETER_COUNT]>;


/// Helper to upload shader parameters
struct ParameterUploader<'a, 'r: 'a> {
    locations: &'a ParameterLocations,
    ll: &'a mut LowLevel,
    resources: &'a mut GuardedResources<'r>,
}

impl<'a, 'r> ShaderParameterVisitor for ParameterUploader<'a, 'r> {
    fn process_f32x16(&mut self, idx: usize, data: &Float32x16) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_MAT4 && size == 1);
                gl_check_error();
                gl!(UniformMatrix4fv(location as i32, size, gl::FALSE, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x4(&mut self, idx: usize, data: &Float32x4) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC4 && size == 1);
                gl_check_error();
                gl!(Uniform4fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x3(&mut self, idx: usize, data: &Float32x3) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC3 && size == 1);
                gl_check_error();
                gl!(Uniform3fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x2(&mut self, idx: usize, data: &Float32x2) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC2 && size == 1);
                gl_check_error();
                gl!(Uniform2fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32(&mut self, idx: usize, data: f32) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT && size == 1);
                gl_check_error();
                gl!(Uniform1fv(location as i32, size, mem::transmute(&data)));
                gl_check_error();
            }
        }
    }

    fn process_tex_2d(&mut self, idx: usize, data: &UnsafeTexture2DIndex) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::SAMPLER_2D && size == 1);
                gl_check_error();
                let texture = &mut self.resources[data];
                let slot = texture.bind(self.ll);
                let slot = slot as u32;
                gl!(Uniform1i(location as i32, slot as i32));
                gl_check_error();
            }
        }
    }

    fn process_attribute(&mut self, idx: usize, data: &UnsafeVertexAttributeHandle) {
        if let ParameterLocation::Attribute { location, type_id, .. } = self.locations[idx] {
            if type_id != 0 {
                gl_check_error();
                let buffer = &mut self.resources[data];
                buffer.bind(self.ll, location, data.1);
                gl_check_error();
            }
        }
    }

    fn process_index(&mut self, _idx: usize, data: &UnsafeIndexBufferIndex) {
        gl_check_error();
        if data.is_null() {
            self.ll.index_binding.bind_no_index();
        } else {
            let buffer = &mut self.resources[data];
            buffer.bind(self.ll);
        }
        gl_check_error();
    }
}


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

    fn draw<'r, 'a, P: ShaderParameters>(&mut self, resources: &'a mut GuardedResources<'r>, ll: &'a mut LowLevel,
                                         parameters: &P, primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
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
            ll: ll,
            resources: resources,
        });
        gl_check_error();

        ll.draw(primitive, vertex_start, vertex_count);
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "Leaking shader program");
    }
}


/// Structure to store the shader abstraction.
impl<DECL: ShaderDeclaration> Resource for ShaderProgramHandle<DECL> {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        struct ReleaseCommand {
            target: UnsafeIndex<GLShaderProgram>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.release(ll);
            }
        }

        queue.add(
            ReleaseCommand {
                target: UnsafeIndex::from_index(&self.0),
            }
        );
    }
}

impl<DECL: ShaderDeclaration> ShaderProgram<DECL> for ShaderProgramHandle<DECL> {
    fn compile<Q: CommandQueue>(&self, queue: &mut Q) {
        /// RenderCommand to allocate the OpenGL program, set the shader sources and compile (link) a shader program
        struct CreateCommand<SD: ShaderDeclaration> {
            target: UnsafeIndex<GLShaderProgram>,
            phantom_sd: PhantomData<SD>,
        }

        impl<SD: ShaderDeclaration> Command for CreateCommand<SD> {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.create_program::<SD>(ll);
                target.parse_parameters::<SD>(ll);
            }
        }

        queue.add(
            CreateCommand::<DECL> {
                target: UnsafeIndex::from_index(&self.0),
                phantom_sd: PhantomData,
            }
        );
    }
    fn draw<Q: CommandQueue>(&self, queue: &mut Q, parameters: DECL::Parameters,
                             primitive: Primitive, vertex_start: usize, vertex_count: usize)
    {
        struct DrawCommand<SD: ShaderDeclaration> {
            target: UnsafeIndex<GLShaderProgram>,
            parameters: SD::Parameters,
            primitive: GLenum,
            vertex_start: GLuint,
            vertex_count: GLuint,
        }

        impl<SD: ShaderDeclaration> Command for DrawCommand<SD> {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                // Without the unsafe code resourecs is borrowed mutable multiple times
                // (once for target and once for the draw function call)
                // As we know (but not the compiler) that, shader won't be accessed
                // during call, it is safe to have a mutable reference into the shader
                let target = unsafe {
                    let a = &mut resources[&self.target] as *mut GLShaderProgram;
                    &mut *a
                };
                target.draw(resources, ll, &self.parameters, self.primitive, self.vertex_start, self.vertex_count);
            }
        }

        queue.add(
            DrawCommand::<DECL> {
                target: UnsafeIndex::from_index(&self.0),
                parameters: parameters,
                primitive: gl_get_primitive_enum(primitive),
                vertex_start: vertex_start as GLuint,
                vertex_count: vertex_count as GLuint,
            }
        );
    }
}

/*
use std::marker::PhantomData;
use backend::*;

use store::handlestore::*;

crate type ShaderProgramStore = Store<ShaderProgramImpl>;
crate type GuardedShaderProgramStore<'a> = UpdateGuardStore<'a, ShaderProgramImpl>;
crate type ShaderProgramIndex = Index<ShaderProgramImpl>;
pub type UnsafeShaderProgramIndex = UnsafeIndex<ShaderProgramImpl>;


/// Handle to a texture 2d resource
#[derive(Clone)]
pub struct ShaderProgramHandle<DECL: ShaderDeclaration>( crate ShaderProgramIndex, PhantomData<DECL>);

impl<DECL: ShaderDeclaration> ShaderProgramHandle<DECL> {
    pub fn null() -> ShaderProgramHandle<DECL> {
        ShaderProgramHandle(ShaderProgramIndex::null(), PhantomData)
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> ShaderProgramHandle<DECL> {
        ShaderProgramHandle(res.resources.shaders.add(ShaderProgramImpl::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn as_ref(&self) -> UnsafeIndex<ShaderProgramImpl> {
        UnsafeIndex::from_index(&self.0)
    }
}

impl<'a, DECL: ShaderDeclaration> From<&'a ShaderProgramHandle<DECL>> for UnsafeIndex<ShaderProgramImpl> {
    #[inline(always)]
    fn from(idx: &ShaderProgramHandle<DECL>) -> UnsafeIndex<ShaderProgramImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}

*/

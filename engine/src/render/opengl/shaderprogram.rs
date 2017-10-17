use std::rc::Rc;
use std::cell::{RefCell};
use std::str::from_utf8;
use std::vec::Vec;
use std::marker::PhantomData;

use arrayvec::ArrayVec;

use render::*;
use render::opengl::lowlevel::*;
use render::opengl::commandqueue::*;

struct ShaderError(String);

/// Converts a ShaderType enum to the corresponding GLenum.
fn gl_get_shader_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}

/// Attribute info
#[derive(Copy, Clone, Debug)]
struct ShaderAttribute {
    location: GLuint,
    size: GLint,
    type_id: GLenum,
}

impl ShaderAttribute {
    fn new() -> ShaderAttribute {
        ShaderAttribute {
            location: 0,
            size: 0,
            type_id: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.type_id != 0
    }
}

type ShaderAttributes = ArrayVec<[ShaderAttribute; MAX_USED_ATTRIBUTE_COUNT]>;


/// Uniform info
#[derive(Copy, Clone, Debug)]
struct ShaderUniform {
    location: GLuint,
    size: GLint,
    type_id: GLenum,

    /// Store the offset of the data in the GLShaderProgramData::uniform_data
    data_offset: usize,
    data_size: usize,
}

impl ShaderUniform {
    fn new() -> ShaderUniform {
        ShaderUniform {
            location: 0,
            size: 0,
            type_id: 0,
            data_offset: usize::max_value(),
            data_size: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.type_id != 0
    }
}

type ShaderUniforms = ArrayVec<[ShaderUniform; MAX_USED_UNIFORM_COUNT]>;


/// Structure to store hardware data associated to a ShaderProgram.
struct GLShaderProgramData {
    hw_id: GLuint,
    attributes: ShaderAttributes,
    uniforms: ShaderUniforms,
    uniform_data: Vec<u8>,
}

impl GLShaderProgramData {
    fn new() -> GLShaderProgramData {
        GLShaderProgramData {
            hw_id: 0,
            attributes: ShaderAttributes::new(),
            uniforms: ShaderUniforms::new(),
            uniform_data: vec!()
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
        unsafe {
            self.hw_id = gl::CreateProgram();
        }

        // create and attach shaders
        gl_check_error();
        let compile_result = SD::map_sources(|(shader_type, source)| {
            let shader_res = self.attach_shader(gl_get_shader_enum(shader_type), source.as_bytes());
            if let Some(ShaderError(msg)) = shader_res.err() {
                println!("shader compilation failed.\nsource:\n{}\nerror:\n{}", source, msg);
                self.release(ll);
                false
            } else {
                true
            }
        });

        assert!(compile_result, "Shader compilation failed");

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

    fn parse_attributes<SD: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
        assert!(SD::Attribute::count() <= MAX_USED_ATTRIBUTE_COUNT, "too many vertex attributes in shader declaration: {}/{}", SD::Attribute::count(), MAX_USED_ATTRIBUTE_COUNT);

        (0..SD::Attribute::count()).for_each(|_| self.attributes.push(ShaderAttribute::new()));

        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut attribute_size: GLint = 0;
        let mut attribute_type: GLenum = 0;

        gl_check_error();
        unsafe {
            gl::GetProgramiv(self.hw_id, gl::ACTIVE_ATTRIBUTES, &mut count);
        }
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_ATTRIBUTE_COUNT, "too many attributes in the shader: {}/{}", count, MAX_USED_ATTRIBUTE_COUNT);

        for location in 0..count {
            gl_check_error();
            unsafe {
                gl::GetActiveAttrib(self.hw_id,
                                    location,
                                    name_buffer.len() as GLint,
                                    &mut name_length,
                                    &mut attribute_size,
                                    &mut attribute_type,
                                    name_buffer.as_ptr() as *mut GLchar);
            }
            gl_check_error();

            let attribute = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let attribute = SD::Attribute::index_from_name(&attribute).expect(&format!("Attribute id could not be resolved for {}", attribute));
            let attribute = &mut self.attributes[attribute];
            attribute.location = location;
            attribute.size = attribute_size;
            attribute.type_id = attribute_type;
            //println!("attribute= {:?}", attribute);
        }
    }

    fn parse_uniforms<SD: ShaderDeclaration>(&mut self, _ll: &mut LowLevel) {
        assert!(SD::Uniform::count() <= MAX_USED_UNIFORM_COUNT, "Too many uniform: {}/{}", SD::Uniform::count(), MAX_USED_UNIFORM_COUNT);

        (0..SD::Uniform::count()).for_each(|_| self.uniforms.push(ShaderUniform::new()));

        let mut count: GLint = 0;
        let name_buffer: [u8; 16] = [0; 16];
        let mut name_length: GLsizei = 0;
        let mut uniform_size: GLint = 0;
        let mut uniform_type: GLenum = 0;

        gl_check_error();
        unsafe {
            gl::GetProgramiv(self.hw_id, gl::ACTIVE_UNIFORMS, &mut count);
        }
        gl_check_error();

        let count = count as GLuint;
        assert!((count as usize) < MAX_USED_UNIFORM_COUNT, "too many uniforms in the shader: {}/{}", count, MAX_USED_UNIFORM_COUNT);

        let mut buffer_size = 0;
        for location in 0..count {
            gl_check_error();
            unsafe {
                gl::GetActiveUniform(self.hw_id,
                                     location,
                                     name_buffer.len() as GLint,
                                     &mut name_length,
                                     &mut uniform_size,
                                     &mut uniform_type,
                                     name_buffer.as_ptr() as *mut GLchar);
            }
            gl_check_error();

            let uniform = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let uniform = SD::Uniform::from_name(&uniform).expect(&format!("Uniform id could not be resolved for {}", uniform));
            println!("uniform= {:?}", uniform);
            let uniform = uniform.to_index();
            let uniform = &mut self.uniforms[uniform];
            uniform.location = location;
            uniform.size = uniform_size;
            uniform.type_id = uniform_type;
            uniform.data_offset = buffer_size;
            println!("uniform= {:?}", uniform);

            buffer_size += uniform.data_size;
        }

        self.uniform_data.resize(buffer_size, 0);
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        gl_check_error();
        ll.program_binding.unbind_if_active(self.hw_id);
        gl_check_error();
        unsafe {
            gl::DeleteProgram(self.hw_id);
        }
        gl_check_error();

        self.hw_id = 0;
        self.attributes.clear();
        self.uniforms.clear();
        self.uniform_data.clear();
    }

    fn draw(&mut self, ll: &mut LowLevel, binding: &GLVertexAttributeVec, primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
        ll.program_binding.bind(self.hw_id);
        for (ref vertex_attrib, ref shader_attrib) in binding.iter().zip(self.attributes.iter()) {
            if shader_attrib.is_valid() {
                vertex_attrib.bind(ll, shader_attrib.location);
            }
        }

        ll.draw(primitive, vertex_start, vertex_count);
    }
}

impl Drop for GLShaderProgramData {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "release shader through a render queue before dropping it" );
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
        shader.parse_attributes::<SD>(ll);
        shader.parse_uniforms::<SD>(ll);
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
struct DrawCommand {
    target: Rc<RefCell<GLShaderProgramData>>,
    binding: GLVertexAttributeVec,
    primitive: GLenum,
    vertex_start: GLuint,
    vertex_count: GLuint,
}

impl Command for DrawCommand {
    fn get_sort_key(&self) -> usize {
        1
    }

    fn process(&mut self, ll: &mut LowLevel) {
        let ref mut shader = *self.target.borrow_mut();
        shader.draw(ll, &self.binding, self.primitive, self.vertex_start, self.vertex_count);
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

    pub fn draw<'a, Q: CommandQueue>(&mut self, queue: &mut Q, binding: GLVertexAttributeVec,
                                     primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        queue.add(
            DrawCommand {
                target: self.0.clone(),
                binding: binding,
                primitive: gl_get_primitive_enum(primitive),
                vertex_start: vertex_start as GLuint,
                vertex_count: vertex_count as GLuint,
            }
        );
    }
}


pub type ShaderProgramImpl = GLShaderProgram;

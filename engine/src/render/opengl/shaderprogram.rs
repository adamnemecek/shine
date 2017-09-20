use std::rc::Rc;
use std::cell::{RefCell};
use std::str::from_utf8;
use std::vec::Vec;
use std::marker::PhantomData;

use render::*;

use render::opengl::lowlevel::*;
use render::opengl::commandqueue::*;

struct ShaderError(String);

struct ShaderSource(GLenum, Vec<u8>);

type ShaderSources = Vec<ShaderSource>;


/// Converts a ShaderType enum to the corresponding GLenum.
fn gl_get_shader_enum(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::FragmentShader => gl::FRAGMENT_SHADER
    }
}

#[derive(Copy, Clone, Debug)]
struct ShaderAttribute {
    location: GLint,
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

    fn isValid(&self) -> bool {
        self.type_id != 0
    }
}


/// Structure to store hardware data associated to a ShaderProgram.
struct GLShaderProgramData {
    hw_id: GLuint,
    attributes: [ShaderAttribute; MAX_BOUND_ATTRIBUTE_COUNT],
}

impl GLShaderProgramData {
    fn new() -> GLShaderProgramData {
        GLShaderProgramData {
            hw_id: 0,
            attributes: [ShaderAttribute::new(); MAX_BOUND_ATTRIBUTE_COUNT],
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

    fn parse_attributes<SA: PrimitiveEnum>(&mut self, _ll: &mut LowLevel) {
        assert!(SA::count() <= MAX_BOUND_ATTRIBUTE_COUNT, "too many vertex attributes");


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
        assert!((count as usize) < MAX_BOUND_ATTRIBUTE_COUNT, "too many attributes, maximum supported attribute count: {}", MAX_BOUND_ATTRIBUTE_COUNT);

        for location in 0..count {
            gl_check_error();
            unsafe {
                gl::GetActiveAttrib(self.hw_id,
                                    location as u32,
                                    name_buffer.len() as i32,
                                    &mut name_length,
                                    &mut attribute_size,
                                    &mut attribute_type,
                                    name_buffer.as_ptr() as *mut GLchar);
            }
            let name = from_utf8(&name_buffer[0..name_length as usize]).unwrap().to_string();
            let attr_enum = SA::from_name(&name).expect(&format!("attribute id could not be resolved for {}", name));
            let attr_index = attr_enum.to_index();
            self.attributes[attr_index].location = location;
            self.attributes[attr_index].size = attribute_size;
            self.attributes[attr_index].type_id = attribute_type;
            println!("{:?}: {:?}", attr_enum, self.attributes[attr_index]);
            gl_check_error();
        }
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
        println!("hwid: {:?}", self.hw_id);
        gl_check_error();
        self.hw_id = 0;
    }
}

impl Drop for GLShaderProgramData {
    fn drop(&mut self) {
        assert! ( self.hw_id == 0, "release shader through a render queue before dropping it" );
    }
}


/// RenderCommand to allocate the OpenGL program, set the shader sources and compile (link) a shader program
struct CreateCommand<SA: PrimitiveEnum> {
    target: Rc<RefCell<GLShaderProgramData>>,
    sources: ShaderSources,
    phantom_sa: PhantomData<SA>,
}

impl<SA: PrimitiveEnum> Command for CreateCommand<SA> {
    fn process(&mut self, ll: &mut LowLevel) {
        let ref mut shader = *self.target.borrow_mut();
        shader.create_program(ll, &mut self.sources);
        shader.parse_attributes::<SA>(ll);
    }
}


/// RenderCommand to release the allocated OpenGL program.
struct ReleaseCommand {
    target: Rc<RefCell<GLShaderProgramData>>,
}

impl Command for ReleaseCommand {
    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
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

    pub fn set_sources<'a, SA: PrimitiveEnum, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) {
        println!("GLShaderProgram - set_sources");
        queue.add(
            CreateCommand::<SA> {
                target: self.0.clone(),
                sources: sources
                    .map(|&(t, s)| ShaderSource(gl_get_shader_enum(t), s.as_bytes().to_vec()))
                    .collect(),
                phantom_sa: PhantomData,
            }
        );
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        println!("GLShaderProgram - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }
}


pub type ShaderProgramImpl = GLShaderProgram;

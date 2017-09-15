mod utils;
mod vertexattribute;
mod vertexbinding;
mod indexbinding;
mod programbinding;

pub use render::opengl::gl;
pub use render::opengl::gl::types::*;

pub use self::utils::*;
pub use self::vertexattribute::*;
use self::vertexbinding::VertexBinding;
use self::indexbinding::IndexBinding;
use self::programbinding::ProgramBinding;

use render::*;

/// Structure to manage the GL state machine at a low level.
///
/// It tries to collect and minimize the states changes between the render calls.
/// by caching the bound attributes. Also it tries to move the API into a stateless
/// direction where one draw call does not effects the next calls.
pub struct LowLevel {
    screen_size: Size,
    pub vertex_binding: VertexBinding,
    pub index_binding: IndexBinding,
    pub program_binding: ProgramBinding,
}

impl LowLevel {
    pub fn new() -> LowLevel {
        LowLevel {
            screen_size: Size::from((0, 0)),
            vertex_binding: VertexBinding::new(),
            index_binding: IndexBinding::new(),
            program_binding: ProgramBinding::new(),
        }
    }

    pub fn release(&mut self) {
        println!("release LowLevel");
    }

    pub fn get_screen_size(&self) -> Size {
        self.screen_size
    }

    pub fn set_screen_size(&mut self, size: Size) {
        self.screen_size = size;
    }

    pub fn start_render(&mut self) {}

    pub fn end_render(&mut self) {}

    /// Draws a geometry using the current states.
    pub fn draw(&mut self, primitive: GLenum, first: GLuint, vertex_count: GLsizei) {
        assert!( self.program_binding.get_bound_id() != 0, "shaderless (fixed function pipeline) rendering is not supported" );
        assert!( (vertex_count % 2 == 0 || primitive != gl::LINES)
            && (vertex_count % 3 == 0 || primitive != gl::TRIANGLES),
        "vertex count does not match primitive type; vertex_count:{} primitive:{} ",
        vertex_count, gl_get_primitive_name(primitive) );


        self.program_binding.commit();
        self.vertex_binding.commit();
        self.index_binding.commit();

        gl_check_error();
        unsafe {
            if !self.index_binding.is_indexed() {
                gl::DrawArrays(primitive, first as GLint, vertex_count);
            } else {
                let offset = self.index_binding.get_offset(first);
                let index_type = self.index_binding.get_index_type();
                gl::DrawElements(primitive, vertex_count, index_type, offset as *const GLvoid);
            }
        }
        gl_check_error();
    }
}

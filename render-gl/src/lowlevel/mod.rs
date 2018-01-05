mod utils;
pub mod vertexbinding;
pub mod indexbinding;
pub mod texturebinding;
pub mod programbinding;

use core::*;
pub use gl;
pub use gl::types::*;
pub use self::utils::*;
pub use self::vertexbinding::VertexBinding;
pub use self::indexbinding::IndexBinding;
pub use self::programbinding::ProgramBinding;
pub use self::texturebinding::TextureBinding;

/// Structure to manage the GL state machine at a low level.
///
/// It tries to collect and minimize the states changes between the render calls.
/// by caching the bound attributes. Also it tries to move the API into a stateless
/// direction where one draw call does not effects the next calls.
pub struct LowLevel {
    screen_size: Size,
    pub vertex_binding: VertexBinding,
    pub index_binding: IndexBinding,
    pub texture_binding: TextureBinding,
    pub program_binding: ProgramBinding,
}

impl LowLevel {
    pub fn new() -> LowLevel {
        LowLevel {
            screen_size: Size::from((0, 0)),
            vertex_binding: VertexBinding::new(),
            index_binding: IndexBinding::new(),
            texture_binding: TextureBinding::new(),
            program_binding: ProgramBinding::new(),
        }
    }

    pub fn release(&mut self) {}

    pub fn get_screen_size(&self) -> Size {
        self.screen_size
    }

    pub fn set_screen_size(&mut self, size: Size) {
        self.screen_size = size;
    }

    pub fn set_forced(&mut self, force: bool) {
        self.vertex_binding.set_forced(force);
        self.index_binding.set_forced(force);
        self.program_binding.set_forced(force);
    }

    pub fn start_render(&mut self) {
        self.set_forced(true);
    }

    pub fn end_render(&mut self) {}

    /// Draws a geometry using the current states.
    pub fn draw(&mut self, primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
        assert!( self.program_binding.get_bound_id() != 0, "shaderless (fixed function pipeline) rendering is not supported" );
        assert!( (vertex_count % 2 == 0 || primitive != gl::LINES)
            && (vertex_count % 3 == 0 || primitive != gl::TRIANGLES),
        "vertex count does not match primitive type; vertex_count:{} primitive:{} ",
        vertex_count, gl_get_primitive_name(primitive) );


        self.program_binding.commit();
        self.vertex_binding.commit();
        self.index_binding.commit();
        self.texture_binding.commit();

        gl_check_error();
        if !self.index_binding.is_indexed() {
            gl!(DrawArrays(primitive,
                           vertex_start as GLint,
                           vertex_count as GLsizei));
        } else {
            let offset = self.index_binding.get_offset(vertex_start);
            let index_type = self.index_binding.get_index_type();
            gl!(DrawElements(primitive,
                             vertex_count as GLsizei,
                             index_type,
                             offset as *const GLvoid));
        }

        gl_check_error();
    }
}

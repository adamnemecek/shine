mod utils;
pub mod vertexbinding;
pub mod indexbinding;
pub mod texturebinding;
pub mod programparameter;
pub mod programbinding;
pub mod states;

mod vertexbuffer;
mod indexbuffer;
mod texture;
mod shaderprogram;

use core::*;

pub use gl;
pub use gl::types::*;
pub use self::utils::*;
pub use self::vertexbinding::*;
pub use self::indexbinding::*;
pub use self::programparameter::*;
pub use self::programbinding::*;
pub use self::texturebinding::*;
pub use self::states::*;

pub use self::indexbuffer::*;
pub use self::vertexbuffer::*;
pub use self::texture::*;
pub use self::shaderprogram::*;


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
    pub states: StateManager,
}

impl LowLevel {
    pub fn new() -> LowLevel {
        let mut ll = LowLevel {
            screen_size: Size::from((0, 0)),
            vertex_binding: VertexBinding::new(),
            index_binding: IndexBinding::new(),
            texture_binding: TextureBinding::new(),
            program_binding: ProgramBinding::new(),
            states: StateManager::new(),
        };

        ll.set_forced(false);

        ll
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
        self.states.set_forced(force);
    }

    pub fn start_render(&mut self) {}

    pub fn end_render(&mut self) {}

    /// Draws a geometry using the current states.
    pub fn draw(&mut self, primitive: GLenum, vertex_start: GLuint, vertex_count: GLuint) {
        assert!(self.program_binding.get_bound_id() != 0, "shaderless (fixed function pipeline) rendering is not supported");
        assert!((vertex_count % 2 == 0 || primitive != gl::LINES)
                    && (vertex_count % 3 == 0 || primitive != gl::TRIANGLES),
                "vertex count does not match primitive type; vertex_count:{} primitive:{} ",
                vertex_count, gl_get_primitive_name(primitive));

        self.states.set_render_size(self.screen_size);

        self.program_binding.commit();
        self.vertex_binding.commit();
        self.index_binding.commit();
        self.texture_binding.commit();
        self.states.commit();

        gl_check_error();
        if !self.index_binding.is_indexed() {
            ffi!(gl::DrawArrays(primitive,
                           vertex_start as GLint,
                           vertex_count as GLsizei));
        } else {
            let offset = self.index_binding.get_offset(vertex_start);
            let index_type = self.index_binding.get_index_type();
            ffi!(gl::DrawElements(primitive,
                             vertex_count as GLsizei,
                             index_type,
                             offset as *const GLvoid));
        }

        gl_check_error();
    }

    /// A simple test function to check if ll is alive
    pub fn hello(&mut self, time: f32) {
        ffi!(gl::ClearColor(time, 0.0, 0.0, 1.0));
        ffi!(gl::Clear(gl::COLOR_BUFFER_BIT));
    }
}

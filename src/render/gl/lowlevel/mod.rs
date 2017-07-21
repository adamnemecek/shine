#![allow(dead_code)]
extern crate gl;

mod vertexbinding;
mod indexbinding;
mod programbinding;

use self::gl::types::*;
use render::gl::utils::*;

use self::vertexbinding::VertexBinding;
use self::indexbinding::IndexBinding;
use self::programbinding::ProgramBinding;

pub struct LowLevel {
    pub vertex_binding: VertexBinding,
    pub index_binding: IndexBinding,
    pub program_binding: ProgramBinding,
}

impl LowLevel {
    pub fn new() -> LowLevel {
        LowLevel {
            vertex_binding: VertexBinding::new(),
            index_binding: IndexBinding::new(),
            program_binding: ProgramBinding::new(),
        }
    }

    pub fn release(&mut self) {
        println!("release LowLevel");
        println!("release LowLevel done");
    }

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

impl Drop for LowLevel {
    fn drop(&mut self) {
        println!("drop LowLevel");
    }
}

#![allow(dead_code)]

use std::mem;

use render::opengl::lowlevel::*;

pub fn gl_check_error() {

}


/// Returns the size (in bytes) of a gl type identified by the type-enum.
pub fn gl_get_type_size(gl_type: GLenum) -> usize {
    match gl_type {
        gl::BYTE => mem::size_of::<GLbyte>(),
        gl::UNSIGNED_BYTE => mem::size_of::<GLubyte>(),
        gl::SHORT => mem::size_of::<GLshort>(),
        gl::UNSIGNED_SHORT => mem::size_of::<GLushort>(),
        gl::INT => mem::size_of::<GLint>(),
        gl::UNSIGNED_INT => mem::size_of::<GLuint>(),
        gl::FLOAT => mem::size_of::<GLfloat>(),
        gl::DOUBLE => mem::size_of::<GLdouble>(),
        _ => panic!("unknown gl type enum{}", gl_type),
    }
}

/// Returns a human readable name for gl type identified by the type-enum.
pub fn gl_get_type_name(gl_type: GLenum) -> &'static str {
    match gl_type {
        gl::BYTE => "Byte",
        gl::UNSIGNED_BYTE => "UnsignedByte",
        gl::SHORT => "Short",
        gl::UNSIGNED_SHORT => "UnsignedShort",
        gl::INT => "Int",
        gl::UNSIGNED_INT => "UnsignedInt",
        gl::FLOAT => "Float",
        gl::DOUBLE => "Double",
        _ => panic!("unknown gl type enum{}", gl_type),
    }
}

/// Returns a human readable name for the gl primitive type identified by the type-enum.
pub fn gl_get_primitive_name(primitive: GLenum) -> &'static str {
    match primitive {
        gl::POINTS => "Points",
        gl::LINES => "Lines",
        gl::TRIANGLES => "Triangles",
        _ => panic!("unknown gl primitive enum{}", primitive),
    }
}


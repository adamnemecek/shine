use std::mem;
use std::ffi::CStr;
use core::*;
use lowlevel::*;
use libconfig::*;


/// Checks the last gl error code
pub fn gl_check_error() {
    if !CHECK_RENDER_ERRORS {
        return;
    }

    match ffi!(gl::GetError()) {
        gl::NO_ERROR => {}

        e @ gl::INVALID_ENUM => { panic!("gl error ({}): GL_INVALID_ENUM, An unacceptable value is specified for an enumerated argument.", e); }
        e @ gl::INVALID_VALUE => { panic!("gl error ({}): GL_INVALID_VALUE, A numeric argument is out of range.", e); }
        e @ gl::INVALID_OPERATION => { panic!("gl error ({}): GL_INVALID_OPERATION, The specified operation is not allowed in the current state.", e); }
        e @ gl::INVALID_FRAMEBUFFER_OPERATION => { panic!("gl error ({}): GL_INVALID_FRAMEBUFFER_OPERATION, The framebuffer object is not complete.", e); }
        e @ gl::OUT_OF_MEMORY => { panic!("gl error ({}): GL_OUT_OF_MEMORY, There is not enough memory left to execute the command.", e); }
        e @ gl::STACK_UNDERFLOW => { panic!("gl error ({}): GL_STACK_UNDERFLOW, An attempt has been made to perform an operation that would cause an internal stack to underflow.", e); }
        e @ gl::STACK_OVERFLOW => { panic!("gl error ({}): GL_STACK_OVERFLOW, An attempt has been made to perform an operation that would cause an internal stack to overflow.", e); }

        e => { panic!("gl error: {}", e); }
    }
}

/// Gets a string aattribute from opengl
pub fn gl_get_string(id: GLenum) -> String {
    gl_check_error();
    let data = ffi!(gl::GetString(id));
    gl_check_error();
    let data = unsafe { CStr::from_ptr(data as *const i8) }.to_bytes().to_vec();
    String::from_utf8(data).unwrap()
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


/// Converts a Primitive enum to the corresponding GLenum.
pub fn glenum_from_primitive(primitive: Primitive) -> GLenum {
    match primitive {
        Primitive::Point => gl::POINT,
        Primitive::Lines => gl::LINE,
        Primitive::Triangles => gl::TRIANGLES,
        Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
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

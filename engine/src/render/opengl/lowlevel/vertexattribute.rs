use render::*;
use render::opengl::lowlevel::*;

pub const MAX_BOUND_ATTRIBUTE_COUNT: usize = 16;

/// Structure to store vertex attribute info for the open GL.
#[derive(Clone, Copy, Debug)]
pub struct VertexAttribute {
    pub component_type: GLenum,
    pub components: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    pub offset: GLintptr
}

impl VertexAttribute {
    pub fn new() -> VertexAttribute {
        VertexAttribute {
            component_type: 0,
            components: 0,
            normalize: 0,
            stride: 0,
            offset: 0
        }
    }

    pub fn new_from_element<T: VertexAttributeInfo>(offset: usize, stride: usize) -> VertexAttribute {
        VertexAttribute {
            component_type: T::get_gl_type_id(),
            components: T::get_component_count(),
            normalize: T::is_fixed_point(),
            stride: stride as GLsizei,
            offset: offset as GLintptr,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.component_type != 0
    }
}

impl PartialEq for VertexAttribute {
    fn eq(&self, other: &VertexAttribute) -> bool {
        return self.component_type == other.component_type
            && self.components == other.components
            && self.normalize == other.normalize
            && self.stride == other.stride
            && self.offset == other.offset
    }
}


/// Trait to help converting an abstract vertex element into VertexAttribute
pub trait VertexAttributeInfo {
    fn get_component_count() -> GLint;
    fn get_gl_type_id() -> GLenum;
    fn is_fixed_point() -> GLboolean;
}


impl VertexAttributeInfo for Float32x4 {
    fn get_component_count() -> GLint { 4 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}


impl VertexAttributeInfo for Float32x3 {
    fn get_component_count() -> GLint { 3 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}


impl VertexAttributeInfo for Float32x2 {
    fn get_component_count() -> GLint { 2 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

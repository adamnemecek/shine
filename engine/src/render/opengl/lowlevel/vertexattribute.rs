use render::*;
use render::opengl::lowlevel::*;

/// Structure to store vertex attribute info for open GL.
#[derive(Clone, Copy, Debug)]
pub struct GLVertexAttributeDescriptor {
    pub component_type: GLenum,
    pub components: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    pub offset: GLintptr
}

impl GLVertexAttributeDescriptor {
    pub fn new() -> GLVertexAttributeDescriptor {
        GLVertexAttributeDescriptor {
            component_type: 0,
            components: 0,
            normalize: 0,
            stride: 0,
            offset: 0
        }
    }

    pub fn new_from_element<T: GLVertexAttributeInfo>(offset: usize, stride: usize) -> GLVertexAttributeDescriptor {
        GLVertexAttributeDescriptor {
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

impl PartialEq for GLVertexAttributeDescriptor {
    fn eq(&self, other: &GLVertexAttributeDescriptor) -> bool {
        return self.component_type == other.component_type
            && self.components == other.components
            && self.normalize == other.normalize
            && self.stride == other.stride
            && self.offset == other.offset
    }
}


/// Trait to help converting an abstract vertex element into GLVertexAttributeDescriptor
pub trait GLVertexAttributeInfo {
    fn get_component_count() -> GLint;
    fn get_gl_type_id() -> GLenum;
    fn is_fixed_point() -> GLboolean;
}

impl GLVertexAttributeInfo for Float32x4 {
    fn get_component_count() -> GLint { 4 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

impl GLVertexAttributeInfo for Float32x3 {
    fn get_component_count() -> GLint { 3 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

impl GLVertexAttributeInfo for Float32x2 {
    fn get_component_count() -> GLint { 2 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

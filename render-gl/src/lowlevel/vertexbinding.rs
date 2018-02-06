use core::*;
use libconfig::*;
use lowlevel::*;


/// Structure to store vertex attribute info for open GL.
#[derive(Clone, Copy, Debug)]
pub struct GLVertexBufferAttribute {
    pub component_type: GLenum,
    pub components: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    pub offset: GLintptr
}

impl GLVertexBufferAttribute {
    pub fn new() -> GLVertexBufferAttribute {
        GLVertexBufferAttribute {
            component_type: 0,
            components: 0,
            normalize: 0,
            stride: 0,
            offset: 0
        }
    }

    pub fn from_layout(layout: &VertexBufferLayoutElement) -> GLVertexBufferAttribute {
        match layout {
            &VertexBufferLayoutElement::Float32 { stride, offset } => GLVertexBufferAttribute {
                component_type: gl::FLOAT,
                components: 1,
                normalize: gl::FALSE,
                stride: stride as GLsizei,
                offset: offset as GLintptr,
            },

            &VertexBufferLayoutElement::Float32x2 { stride, offset } => GLVertexBufferAttribute {
                component_type: gl::FLOAT,
                components: 2,
                normalize: gl::FALSE,
                stride: stride as GLsizei,
                offset: offset as GLintptr,
            },

            &VertexBufferLayoutElement::Float32x3 { stride, offset } => GLVertexBufferAttribute {
                component_type: gl::FLOAT,
                components: 3,
                normalize: gl::FALSE,
                stride: stride as GLsizei,
                offset: offset as GLintptr,
            },

            &VertexBufferLayoutElement::Float32x4 { stride, offset } => GLVertexBufferAttribute {
                component_type: gl::FLOAT,
                components: 4,
                normalize: gl::FALSE,
                stride: stride as GLsizei,
                offset: offset as GLintptr,
            },
        }
    }

    pub fn is_valid(&self) -> bool {
        self.component_type != 0
    }
}

impl PartialEq for GLVertexBufferAttribute {
    fn eq(&self, other: &GLVertexBufferAttribute) -> bool {
        return self.component_type == other.component_type
            && self.components == other.components
            && self.normalize == other.normalize
            && self.stride == other.stride
            && self.offset == other.offset;
    }
}


/// The current attribute bound for the GL
#[derive(Clone, Copy)]
struct BoundVertexAttribute {
    hw_id: GLuint,
    attribute: GLVertexBufferAttribute,
    is_used: bool,
}

impl BoundVertexAttribute {
    fn new() -> BoundVertexAttribute {
        BoundVertexAttribute {
            hw_id: 0,
            attribute: GLVertexBufferAttribute::new(),
            is_used: false,
        }
    }
}


/// Handle vertex binding states
pub struct VertexBinding {
    force: bool,
    bound_id: GLuint,
    bound_attributes: [BoundVertexAttribute; MAX_USED_ATTRIBUTE_COUNT],
}

impl VertexBinding {
    pub fn new() -> VertexBinding {
        VertexBinding {
            force: false,
            bound_id: 0,
            bound_attributes: [BoundVertexAttribute::new(); MAX_USED_ATTRIBUTE_COUNT],
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_forced(&mut self, force: bool) {
        self.force = force;
    }

    /// Binds a vertex buffer for modification.
    pub fn bind_buffer(&mut self, hw_id: GLuint) {
        if !self.force && self.bound_id == hw_id {
            return;
        }

        gl_check_error();
        ffi!(gl::BindBuffer(gl::ARRAY_BUFFER, hw_id));
        gl_check_error();
        self.bound_id = hw_id;
    }

    /// Binds a vertex attribute to the given location.
    pub fn bind_attribute(&mut self, location: GLuint, hw_id: GLuint, attribute: &GLVertexBufferAttribute) {
        assert!(hw_id != 0);

        let attr = &mut self.bound_attributes[location as usize];
        assert!(!attr.is_used, "Vertex location ({}) already bound for the draw call", location);
        attr.is_used = true;

        gl_check_error();
        if self.force || attr.hw_id != hw_id || attr.attribute != *attribute {
            // bind changed attributes
            if self.force || self.bound_id != hw_id {
                ffi!(gl::BindBuffer(gl::ARRAY_BUFFER, hw_id));
                self.bound_id = hw_id;
            }

            ffi!(gl::VertexAttribPointer(location,
                                     attribute.components, attribute.component_type, attribute.normalize,
                                     attribute.stride, attribute.offset as *const GLvoid));
            ffi!(gl::EnableVertexAttribArray(location));
            attr.hw_id = hw_id;
            attr.attribute = *attribute;
            gl_check_error();
        }
    }

    /// Unbinds a vertex buffer if it is active. This function is mainly used during release.
    pub fn unbind_if_active(&mut self, hw_id: GLuint) {
        if self.bound_id == hw_id {
            self.bind_buffer(0);
        }
    }

    /// Finalizes the vertex attribute binding. If an attribute was not bound for the
    /// cuurent render darw, it is disabled automatically.
    pub fn commit(&mut self) {
        gl_check_error();
        for (location, attr) in self.bound_attributes.iter_mut().enumerate() {
            if !attr.is_used && attr.hw_id != 0 {
                // active attributes not bound for this call are disabled automatically
                attr.attribute = GLVertexBufferAttribute::new();
                attr.hw_id = 0;
                ffi!(gl::DisableVertexAttribArray(location as GLuint));
                gl_check_error();
            }
            attr.is_used = false;
        }
    }
}

use render::opengl::lowlevel::*;


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

    pub fn new_from_element<T: GLVertexAttributeTypeInfo>(offset: usize, stride: usize) -> GLVertexBufferAttribute {
        GLVertexBufferAttribute {
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

impl PartialEq for GLVertexBufferAttribute {
    fn eq(&self, other: &GLVertexBufferAttribute) -> bool {
        return self.component_type == other.component_type
            && self.components == other.components
            && self.normalize == other.normalize
            && self.stride == other.stride
            && self.offset == other.offset
    }
}


/// The current attribute bound for the GL
#[derive(Clone, Copy)]
struct BoundVertexAttribute {
    hw_id: GLuint,
    attribute: GLVertexBufferAttribute,
    time_stamp: u8,
}

impl BoundVertexAttribute {
    fn new() -> BoundVertexAttribute {
        BoundVertexAttribute {
            hw_id: 0,
            attribute: GLVertexBufferAttribute::new(),
            time_stamp: 0,
        }
    }
}


/// Handle vertex binding states
pub struct VertexBinding {
    force: bool,
    bound_id: GLuint,
    bound_attributes: [BoundVertexAttribute; MAX_USED_ATTRIBUTE_COUNT],
    time_stamp: u8,
}

impl VertexBinding {
    pub fn new() -> VertexBinding {
        VertexBinding {
            force: false,
            bound_id: 0,
            bound_attributes: [BoundVertexAttribute::new(); MAX_USED_ATTRIBUTE_COUNT],
            time_stamp: 1,
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
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, hw_id);
        }
        gl_check_error();
        self.bound_id = hw_id;
    }

    /// Binds a vertex attribute to the given location.
    pub fn bind_attribute(&mut self, location: GLuint, hw_id: GLuint, attribute: &GLVertexBufferAttribute) {
        assert!( hw_id != 0 );

        let attr = &mut self.bound_attributes[location as usize];
        assert!(attr.time_stamp != self.time_stamp, "Vertex attribute ({}) already bound for drawing", location);
        gl_check_error();
        if self.force || attr.hw_id != hw_id || attr.attribute != *attribute {
            //bind buffer
            unsafe {
                if self.force || self.bound_id == hw_id {
                    gl::BindBuffer(gl::ARRAY_BUFFER, hw_id);
                    gl::VertexAttribPointer(location,
                                            attribute.components, attribute.component_type, attribute.normalize,
                                            attribute.stride, attribute.offset as *const GLvoid);
                    gl::EnableVertexAttribArray(location);
                }
            }
            self.bound_id = hw_id;
            attr.hw_id = hw_id;
            attr.attribute = *attribute;
            attr.time_stamp = self.time_stamp;
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
            if attr.time_stamp != self.time_stamp && attr.hw_id != 0 {
                // active attributes not bound for this call are disabled automatically
                attr.attribute = GLVertexBufferAttribute::new();
                attr.hw_id = 0;
                unsafe {
                    gl::DisableVertexAttribArray(location as GLuint);
                }
                gl_check_error();
            }
        }

        self.time_stamp = self.time_stamp.wrapping_add(1);
    }
}


/// Trait to help converting an vertex attribute type into GLVertexBufferAttribute
pub trait GLVertexAttributeTypeInfo {
    fn get_component_count() -> GLint;
    fn get_gl_type_id() -> GLenum;
    fn is_fixed_point() -> GLboolean;
}

impl GLVertexAttributeTypeInfo for Float32x4 {
    fn get_component_count() -> GLint { 4 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

impl GLVertexAttributeTypeInfo for Float32x3 {
    fn get_component_count() -> GLint { 3 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

impl GLVertexAttributeTypeInfo for Float32x2 {
    fn get_component_count() -> GLint { 2 }
    fn get_gl_type_id() -> GLenum { gl::FLOAT }
    fn is_fixed_point() -> GLboolean { gl::FALSE }
}

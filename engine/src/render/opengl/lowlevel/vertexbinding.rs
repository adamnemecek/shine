use render::opengl::lowlevel::*;


#[derive(Clone, Copy)]
struct BoundVertexAttribute {
    hw_id: GLuint,
    attribute: GLVertexAttributeDescriptor,
    time_stamp: u8,
}

impl BoundVertexAttribute {
    fn new() -> BoundVertexAttribute {
        BoundVertexAttribute {
            hw_id: 0,
            attribute: GLVertexAttributeDescriptor::new(),
            time_stamp: 0,
        }
    }
}


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
    pub fn bind_attribute(&mut self, location: GLuint, hw_id: GLuint, attribute: &GLVertexAttributeDescriptor) {
        assert!( hw_id != 0 );

        let attr = &mut self.bound_attributes[location as usize];
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
        for (location, attr) in self.bound_attributes.iter_mut().enumerate() {
            if attr.time_stamp != self.time_stamp && attr.hw_id != 0 {
                // untouched attributes are unbound automatically
                attr.attribute = GLVertexAttributeDescriptor::new();
                attr.hw_id = 0;
                unsafe {
                    gl::DisableVertexAttribArray(location as GLuint);
                }
                gl_check_error();
            }
        }

        self.time_stamp.wrapping_add(1);
    }
}
#![allow(dead_code)]
extern crate gl;

use self::gl::types::{GLenum, GLint, GLuint, GLboolean, GLsizei, GLintptr, GLvoid};
use super::utils::gl_check_error;

const MAX_ATTRIBUTE_COUNT: usize = 16;

#[derive(Clone, Copy)]
pub struct VertexAttribute {
    gl_type: GLenum,
    components: GLint,
    normalize: GLboolean,
    stride: GLsizei,
    offset: GLintptr
}

impl VertexAttribute {
    fn new() -> VertexAttribute {
        VertexAttribute {
            gl_type: 0,
            components: 0,
            normalize: 0,
            stride: 0,
            offset: 0
        }
    }
}

impl PartialEq for VertexAttribute {
    fn eq(&self, other: &VertexAttribute) -> bool {
        return self.gl_type == other.gl_type
            && self.components == other.components
            && self.normalize == other.normalize
            && self.stride == other.stride
            && self.offset == other.offset
    }
}


#[derive(Clone, Copy)]
struct BoundVertexAttribute
{
    time_stamp: u8,
    dirty: bool,
    buffer_id: GLuint,
    attribute: VertexAttribute,
}

impl BoundVertexAttribute {
    fn new() -> BoundVertexAttribute {
        BoundVertexAttribute {
            time_stamp: 0,
            dirty: false,
            buffer_id: 0,
            attribute: VertexAttribute::new(),
        }
    }
}


pub struct VertexBinding {
    force: bool,
    time_stamp: u8,
    bound_id: GLuint,
    bound_attributes: [BoundVertexAttribute; MAX_ATTRIBUTE_COUNT],
}

impl VertexBinding {
    pub fn new() -> VertexBinding {
        VertexBinding {
            force: false,
            time_stamp: 1,
            bound_id: 0,
            bound_attributes: [BoundVertexAttribute::new(); MAX_ATTRIBUTE_COUNT],
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_force(&mut self, force: bool) {
        self.force = force;
    }

    /// Binds a vertex buffer
    pub fn bind_buffer(&mut self, buffer_id: GLuint) {
        if !self.force && self.bound_id == buffer_id {
            return;
        }

        gl_check_error();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
        }
        gl_check_error();
        self.bound_id = buffer_id;
    }

    /// Binds a vertex attribute to the given location.
    pub fn bind_attribute(&mut self, location: GLuint, buffer_id: GLuint, attribute: &VertexAttribute) {
        assert!( buffer_id != 0 );

        let attr = &mut self.bound_attributes[location as usize];
        attr.time_stamp = self.time_stamp; // make it dirty
        if attr.attribute == *attribute && attr.buffer_id == buffer_id {
            return;
        }
        attr.attribute = *attribute;
        attr.buffer_id = buffer_id;
        attr.dirty = true;
    }

    /// Unbinds a vertex buffer if it is active. This function is usualy used during release.
    pub fn unbind_if_active(&mut self, buffer_id: GLuint) {
        if self.bound_id == buffer_id {
            self.bind_buffer(0);
        }
    }

    /// Finalizes the vertex attributes and commits all vertex related pending GL calls.
    /// Attributes not bound since the last render call are automatically disabled.
    pub fn commit(&mut self) {
        for (attr_id, attr) in self.bound_attributes.iter_mut().enumerate() {
            if attr.time_stamp != self.time_stamp && attr.buffer_id != 0 {
                // untouched attributes are unbound automatically
                attr.attribute = VertexAttribute::new();
                attr.buffer_id = 0;
                attr.dirty = true;
            }
            attr.time_stamp == self.time_stamp;

            if attr.dirty || self.force {
                attr.dirty = false;
                gl_check_error();
                unsafe {
                    let location = attr_id as GLuint;
                    if attr.buffer_id == 0 {
                        gl::DisableVertexAttribArray(location);
                    } else {
                        if self.force || self.bound_id != attr.buffer_id {
                            gl::BindBuffer(gl::ARRAY_BUFFER, attr.buffer_id);
                            self.bound_id = attr.buffer_id;
                        }
                        gl::VertexAttribPointer(location,
                                                attr.attribute.components, attr.attribute.gl_type, attr.attribute.normalize,
                                                attr.attribute.stride, attr.attribute.offset as *const GLvoid);
                        gl::EnableVertexAttribArray(location);
                    }
                }
                gl_check_error();
            }
        }

        self.time_stamp += 1;
    }
}
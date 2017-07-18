#![allow(dead_code)]
extern crate gl;

use self::gl::types::{GLenum, GLuint};
use super::utils::{gl_check_error, gl_get_type_size};

#[derive(Clone, Copy)]
struct BoundIndex
{
    time_stamp: u8,
    buffer_id: GLuint,
    index_type: GLenum,
}

impl BoundIndex {
    fn new() -> BoundIndex {
        BoundIndex {
            time_stamp: 0,
            buffer_id: 0,
            index_type: 0,
        }
    }
}


pub struct IndexBinding {
    force: bool,
    time_stamp: u8,
    bound_id: GLuint,
    bound_index: BoundIndex,
}

impl IndexBinding {
    pub fn new() -> IndexBinding {
        IndexBinding {
            force: false,
            time_stamp: 1,
            bound_id: 0,
            bound_index: BoundIndex::new(),
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_force(&mut self, force: bool) {
        self.force = force;
    }

    /// Returns if index buffer is bound
    pub fn is_indexed(&self) -> bool {
        self.bound_index.buffer_id != 0
    }

    /// Returns the type of the bound index
    pub fn get_index_type(&self) -> GLuint {
        assert!( self.is_indexed() && self.bound_index.index_type != 0 );
        self.bound_index.index_type
    }

    /// Calculates the offset (in bytes) for the given start vertex
    pub fn get_offset(&self, first: GLuint) -> usize {
        assert!( self.is_indexed() && self.bound_index.index_type != 0 );
        gl_get_type_size(self.bound_index.index_type) * (first as usize)
    }

    /// Binds an index buffer.
    pub fn bind_buffer(&mut self, buffer_id: GLuint) {
        if !self.force && self.bound_id == buffer_id {
            return;
        }

        gl_check_error();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
        }
        gl_check_error();
        self.bound_id = buffer_id;
    }

    /// Prepares for rendering without index buffer.
    pub fn bind_no_index(&mut self) {
        self.bound_index.time_stamp = self.time_stamp;
        if self.bound_index.buffer_id == 0 {
            return;
        }

        self.bound_index.buffer_id = 0;
        self.bound_index.index_type = 0;
    }

    /// Prepares for rendering with index buffer.
    pub fn bind_index(&mut self, buffer_id: GLuint, index_type: GLenum) {
        assert!(buffer_id != 0);
        assert!(index_type != 0);

        self.bound_index.time_stamp = self.time_stamp;
        if self.bound_index.buffer_id == buffer_id && self.bound_index.index_type == index_type {
            return;
        }

        self.bound_index.buffer_id = buffer_id;
        self.bound_index.index_type = index_type;
    }

    /// Unbinds an index buffer if it is active. This function is usualy used during release.
    pub fn unbind_if_active(&mut self, buffer_id: GLuint) {
        if self.bound_id == buffer_id {
            self.bind_buffer(0);
        }
    }

    /// Finalizes the index binding and commits all index related GL calls.
    /// If indexed rendering is not enabled since the last render call,
    /// (ex. bind_index was not called) non-index rendering is assumed.
    pub fn commit(&mut self) {
        if self.bound_index.time_stamp != self.time_stamp {
            self.bind_no_index();
        }

        let buffer_id = self.bound_index.buffer_id;
        self.bind_buffer(buffer_id);
        self.time_stamp += 1;
    }
}

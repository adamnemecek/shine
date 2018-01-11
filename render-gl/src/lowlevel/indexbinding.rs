use lowlevel::*;

/// The current index buffer bound for the GL
#[derive(Clone, Copy)]
struct BoundIndex
{
    time_stamp: u8,
    hw_id: GLuint,
    index_type: GLenum,
}

impl BoundIndex {
    fn new() -> BoundIndex {
        BoundIndex {
            time_stamp: 0,
            hw_id: 0,
            index_type: 0,
        }
    }
}


/// Handle index binding states
pub struct IndexBinding {
    force: bool,
    time_stamp: u8,
    bound_index: BoundIndex,
}

impl IndexBinding {
    pub fn new() -> IndexBinding {
        IndexBinding {
            force: false,
            time_stamp: 1,
            bound_index: BoundIndex::new(),
        }
    }

    /// Returns the corresponding GLenum from an IndexBufferLayout
    pub fn glenum_from_index_type(l: IndexBufferLayout) -> GLenum {
        match l {
            IndexBufferLayout::U8 => gl::UNSIGNED_BYTE,
            IndexBufferLayout::U16 => gl::UNSIGNED_SHORT,
            IndexBufferLayout::U32 => gl::UNSIGNED_INT,
        }
    }

    /// Enables/Disables the forced state changed. When enabled, the cached state is ignored
    /// and gl commands are always generated.
    pub fn set_forced(&mut self, force: bool) {
        self.force = force;
    }

    /// Returns if index buffer is bound
    pub fn is_indexed(&self) -> bool {
        self.bound_index.hw_id != 0
    }

    /// Returns the type id of the bound index
    pub fn get_index_type(&self) -> GLenum {
        assert!(self.is_indexed() && self.bound_index.index_type != 0);
        self.bound_index.index_type
    }

    /// Calculates the offset (in bytes) for the given start vertex
    pub fn get_offset(&self, first: GLuint) -> usize {
        assert!(self.is_indexed() && self.bound_index.index_type != 0);
        gl_get_type_size(self.bound_index.index_type) * (first as usize)
    }

    /// Binds an index buffer for modification
    pub fn bind_buffer(&mut self, hw_id: GLuint) {
        assert!(self.bound_index.time_stamp != self.time_stamp, "Index already bound for drawing");
        if !self.force && self.bound_index.hw_id == hw_id {
            return;
        }

        gl_check_error();
        ugl!(BindBuffer(gl::ELEMENT_ARRAY_BUFFER, hw_id));
        gl_check_error();
        self.bound_index.hw_id = hw_id;
        self.bound_index.index_type = 0;
    }

    /// Sets up states for rendering without index buffer.
    pub fn bind_no_index(&mut self) {
        assert!(self.bound_index.time_stamp != self.time_stamp, "Index already bound for drawing");
        self.bind_buffer(0);
        self.bound_index.index_type = 0;
        self.bound_index.time_stamp = self.time_stamp;
    }

    /// Sets up states for rendering with index buffer.
    pub fn bind_index(&mut self, hw_id: GLuint, index_type: GLenum) {
        assert!(hw_id != 0);
        assert!(index_type != 0);
        assert!(self.bound_index.time_stamp != self.time_stamp, "Index already bound for drawing");
        self.bind_buffer(hw_id);
        self.bound_index.index_type = index_type;
        self.bound_index.time_stamp = self.time_stamp;
    }

    /// Unbinds an index buffer if it is active. This function is mainly used during release.
    pub fn unbind_if_active(&mut self, hw_id: GLuint) {
        assert!(self.bound_index.time_stamp != self.time_stamp, "Index already bound for drawing");
        if self.bound_index.hw_id == hw_id {
            self.bind_buffer(0);
        }
    }

    /// Finalizes the index binding. If no index binding was bound since the last
    /// commit, non-index rendering is assumed.
    pub fn commit(&mut self) {
        if self.bound_index.time_stamp != self.time_stamp {
            self.bind_no_index();
        }
        self.time_stamp = self.time_stamp.wrapping_add(1);
    }
}

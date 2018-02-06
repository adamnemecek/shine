use lowlevel::*;

/// The current index buffer bound for the GL
#[derive(Clone, Copy)]
struct BoundIndex
{
    hw_id: GLuint,
    index_type: GLenum,
    is_used: bool,
}

impl BoundIndex {
    fn new() -> BoundIndex {
        BoundIndex {
            hw_id: 0,
            index_type: 0,
            is_used: false,
        }
    }
}


/// Handle index binding states
pub struct IndexBinding {
    force: bool,
    bound_index: BoundIndex,
}

impl IndexBinding {
    pub fn new() -> IndexBinding {
        IndexBinding {
            force: false,
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
        assert!(!self.bound_index.is_used, "Index already bound for drawing, cannot rebind a new buffer");
        if !self.force && self.bound_index.hw_id == hw_id {
            return;
        }

        gl_check_error();
        ffi!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, hw_id));
        gl_check_error();
        self.bound_index.hw_id = hw_id;
        // disable any index type info, as we've no info right now
        // This function is used to bind buffer for data upload,
        // the bind_no_index and bind_index shall be used for render binding
        self.bound_index.index_type = 0;
    }

    /// Sets up states for rendering without index buffer.
    pub fn bind_no_index(&mut self) {
        self.bind_buffer(0);
        self.bound_index.is_used = true;
        self.bound_index.index_type = 0;
    }

    /// Sets up states for rendering with index buffer.
    pub fn bind_index(&mut self, hw_id: GLuint, index_type: GLenum) {
        assert!(hw_id != 0);
        assert!(index_type != 0);
        self.bind_buffer(hw_id);
        self.bound_index.is_used = true;
        self.bound_index.index_type = index_type;
    }

    /// Unbinds an index buffer if it is active. This function is mainly used during release.
    pub fn unbind_if_active(&mut self, hw_id: GLuint) {
        assert!(!self.bound_index.is_used, "Index already bound for drawing, cannot unbind the buffer");
        if self.bound_index.hw_id == hw_id {
            self.bind_buffer(0);
        }
    }

    /// Finalizes the index binding. If no index binding was bound since the last
    /// commit, non-index rendering is assumed.
    pub fn commit(&mut self) {
        if !self.bound_index.is_used {
            self.bind_no_index();
        }
        self.bound_index.is_used = false;
    }
}

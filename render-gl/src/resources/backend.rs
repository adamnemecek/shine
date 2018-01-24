use core::*;
use lowlevel::*;


/// Render backend implementation using opengl
pub struct GLBackend {
    ll: LowLevel,

    //index_store: IndexBufferStore,
}

impl GLBackend {
    pub fn new() -> GLBackend {
        GLBackend {
            ll: LowLevel::new(),
            //vertex_store: GLVertexStore::new(),
        }
    }

    pub fn get_screen_size(&self) -> Size {
        self.ll.get_screen_size()
    }

    pub fn set_screen_size(&mut self, size: Size) {
        self.ll.set_screen_size(size);
    }

    pub fn start_render(&mut self) -> Result<(), Error> {
        self.ll.start_render();
        Ok(())
    }

    pub fn end_render(&mut self) -> Result<(), Error> {
        self.ll.end_render();
        Ok(())
    }

    pub fn ll_mut(&mut self) -> &mut LowLevel {
        &mut self.ll
    }
}


/// Frame compose implementation for opengl.
pub struct GLFrameCompose {
    ll: *mut LowLevel,
}

impl GLFrameCompose {
    pub fn ll_mut(&self) -> &mut LowLevel {
        unsafe { &mut *self.ll }
    }
}

impl FrameCompose for GLFrameCompose {
    fn flush(&mut self) {}
}

impl Backend for GLBackend {
    type FrameCompose = GLFrameCompose;

    fn compose(&mut self) -> GLFrameCompose {
        GLFrameCompose {
            ll: &mut self.ll
        }
    }
}

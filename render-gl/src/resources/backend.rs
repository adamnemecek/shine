use core::*;
use lowlevel::*;
use resources::*;

/// Guarded backend connection to collect command messages
pub struct GLFrameComposer {
    command_queue: CommandProduceGuard<'static>,
    index_store: ReadGuardIndexBuffer<'static>,
    vertex_store: ReadGuardVertexBuffer<'static>,
    texture_2d_store: ReadGuardTexture2D<'static>,
    shader_program_store: ReadGuardShaderProgram<'static>,
}

impl GLFrameComposer {
    pub fn add_command<C: Into<Command>>(&mut self, order: u32, cmd: C) {
        self.command_queue.add(CommandOrder(0, order), cmd.into());
    }

    pub fn add_index_buffer(&mut self, data: GLIndexBuffer) -> IndexBufferIndex {
        self.index_store.add(data)
    }

    pub fn add_vertex_buffer(&mut self, data: GLVertexBuffer) -> VertexBufferIndex {
        self.vertex_store.add(data)
    }

    pub fn add_texture_2d(&mut self, data: GLTexture) -> Texture2DIndex {
        self.texture_2d_store.add(data)
    }

    pub fn add_shader_program(&mut self, data: GLShaderProgram) -> ShaderProgramIndex {
        self.shader_program_store.add(data)
    }
}


/// Guarded backend connection to process command messages
pub struct GLFrameFlusher<'a> {
    pub index_store: WriteGuardIndexBuffer<'a>,
    pub vertex_store: WriteGuardVertexBuffer<'a>,
    pub texture_2d_store: WriteGuardTexture2D<'a>,
    pub shader_program_store: WriteGuardShaderProgram<'a>,
}


/// Render backend implementation using opengl
pub struct GLBackend {
    ll: LowLevel,
    command_store: CommandStore,
    index_store: IndexBufferStore,
    vertex_store: VertexBufferStore,
    texture_2d_store: Texture2DStore,
    shader_program_store: ShaderProgramStore,
}

impl GLBackend {
    pub fn new() -> GLBackend {
        GLBackend {
            ll: LowLevel::new(),
            command_store: CommandStore::new(),
            index_store: IndexBufferStore::new(),
            vertex_store: VertexBufferStore::new(),
            texture_2d_store: Texture2DStore::new(),
            shader_program_store: ShaderProgramStore::new(),
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

impl Backend for GLBackend {
    type FrameCompose/*<'a>*/ = GLFrameComposer/*<'a>*/;

    fn compose<'a>(&'a self) -> GLFrameComposer/*<'a>*/ {
        use std::mem;
        // compose shall not outlive Backend but cannot enforce without generic_associated_types
        // so unsafe code is used with 'static lifetime

        unsafe {
            GLFrameComposer {
                command_queue: mem::transmute(self.command_store.produce()),
                index_store: mem::transmute(self.index_store.read()),
                vertex_store: mem::transmute(self.vertex_store.read()),
                texture_2d_store: mem::transmute(self.texture_2d_store.read()),
                shader_program_store: mem::transmute(self.shader_program_store.read()),
            }
        }
    }

    fn flush(&mut self) {
        let mut consume = self.command_store.consume(|&k| (k.0 as u64) << 32 + k.1 as u64);
        let mut flush = GLFrameFlusher {
            index_store: self.index_store.write(),
            vertex_store: self.vertex_store.write(),
            texture_2d_store: self.texture_2d_store.write(),
            shader_program_store: self.shader_program_store.write(),
        };

        for cmd in consume.drain() {
            cmd.process(&mut self.ll, &mut flush);
        }

        // release unreferenced resources
        let ref mut ll = self.ll;
        flush.index_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        flush.vertex_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        flush.texture_2d_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        flush.shader_program_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
    }
}


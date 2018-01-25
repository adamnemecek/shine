use core::*;
use lowlevel::*;
use resources::*;

/// Guarded backend connection to collect command messages
pub struct GLFrameComposer {
    command_queue: CommandProduceGuard<'static>,
    index_store: ReadGuardIndexBuffer<'static>,
    vertex_store: ReadGuardVertexBuffer<'static>,
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
}


/// Guarded backend connection to process command messages
pub struct GLFrameFlusher<'a> {
    pub index_store: WriteGuardIndexBuffer<'a>,
    pub vertex_store: WriteGuardVertexBuffer<'a>,
}


/// Render backend implementation using opengl
pub struct GLBackend {
    ll: LowLevel,
    command_store: CommandStore,
    index_store: IndexBufferStore,
    vertex_store: VertexBufferStore,
}

impl GLBackend {
    pub fn new() -> GLBackend {
        GLBackend {
            ll: LowLevel::new(),
            command_store: CommandStore::new(),
            index_store: IndexBufferStore::new(),
            vertex_store: VertexBufferStore::new(),
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
            }
        }
    }

    fn flush(&mut self) {
        let mut consume = self.command_store.consume(|&k| (k.0 as u64) << 32 + k.1 as u64);
        let mut flush = GLFrameFlusher {
            index_store: self.index_store.write(),
            vertex_store: self.vertex_store.write(),
        };

        for cmd in consume.drain() {
            cmd.process(&mut self.ll, &mut flush);
        }

        // release unreferenced resources
        let ref mut ll = self.ll;
        flush.index_store.retain(|data, referenced| {
            if !referenced { data.release(ll); }
            false
        });
        flush.vertex_store.retain(|data, referenced| {
            if !referenced { data.release(ll); }
            false
        });
    }
}


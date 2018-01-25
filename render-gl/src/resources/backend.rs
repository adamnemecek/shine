use core::*;
use lowlevel::*;
use resources::*;

/// Guarded backend connection to build command messages
pub struct GLFrameCompose {
    pub command_queue: CommandProduceGuard<'static>,
    pub index_store: ReadGuardIndexBuffer<'static>,
}

pub struct GLFrameFlush<'a> {
    pub index_store: WriteGuardIndexBuffer<'a>,
}


/// Render backend implementation using opengl
pub struct GLBackend {
    ll: LowLevel,

    command_store: CommandStore,
    index_store: IndexBufferStore,
}

impl GLBackend {
    pub fn new() -> GLBackend {
        GLBackend {
            ll: LowLevel::new(),
            command_store: CommandStore::new(),
            index_store: IndexBufferStore::new(),
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

    pub fn hello(&mut self, time: f32) {
        {
            let produce = self.command_store.produce();
            produce.add(CommandOrder(0, 0), Command::Hello { time: time });
        }
        self.flush();
    }
}

impl Backend for GLBackend {
    type FrameCompose/*<'a>*/ = GLFrameCompose/*<'a>*/;

    fn compose<'a>(&'a self) -> GLFrameCompose/*<'a>*/ {
        use std::mem;
        // compose shall not outlive Backend but cannot enforce without generic_associated_types
        // so unsafe code is used with 'static lifetime

        unsafe {
            GLFrameCompose {
                command_queue: mem::transmute(self.command_store.produce()),
                index_store: mem::transmute(self.index_store.read()),
            }
        }
    }

    fn flush(&mut self) {
        let mut consume = self.command_store.consume(|&k| (k.0 as u64) << 32 + k.1 as u64);
        let mut flush = GLFrameFlush {
            index_store: self.index_store.write(),
        };

        for cmd in consume.drain() {
            cmd.process(&mut self.ll, &mut flush);
        }
    }
}

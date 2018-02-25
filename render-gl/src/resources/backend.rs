use core::*;
use lowlevel::*;
use resources::*;

/// Guarded backend connection to collect command messages
pub struct GLCommandQueue {
    command_queue: CommandProduceGuard<'static>,
    index_store: ReadGuardIndexBuffer<'static>,
    vertex_store: ReadGuardVertexBuffer<'static>,
    texture_2d_store: ReadGuardTexture2D<'static>,
    shader_program_store: ReadGuardShaderProgram<'static>,
}

impl GLCommandQueue {
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
pub struct GLCommandProcessContext/*<'a>*/ {
    pub ll: &'static mut LowLevel,
    pub index_store: WriteGuardIndexBuffer<'static>,
    pub vertex_store: WriteGuardVertexBuffer<'static>,
    pub texture_2d_store: WriteGuardTexture2D<'static>,
    pub shader_program_store: WriteGuardShaderProgram<'static>,
}


/// Clear command to set up view for rendering
pub struct ClearCommand {
    color: Option<Float32x4>,
    depth: Option<f32>,
    //stencil: Option<u32>
    viewport: Option<Viewport>,
}

impl ClearCommand {
    pub fn process(&mut self, context: &mut GLCommandProcessContext) {
        context.ll.init_view(self.viewport, self.color, self.depth);
    }
}

impl From<ClearCommand> for Command {
    #[inline(always)]
    fn from(value: ClearCommand) -> Command {
        Command::Clear(value)
    }
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

unsafe impl Sync for LowLevel {}

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
    type CommandQueue/*<'a>*/ = GLCommandQueue/*<'a>*/;
    type CommandContext/*<'a>*/ = GLCommandProcessContext/*<'a>*/;
    type VertexBufferLayout = GLVertexBufferLayout;

    fn get_queue<'a>(&'a self) -> GLCommandQueue/*<'a>*/ {
        /*generic_associated_types workaround*/ unsafe {
            // compose shall not outlive Backend but cannot enforce without generic_associated_types
            // so this 'static lifetime workaround is used
            use std::mem;
            GLCommandQueue {
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
        let mut context = {
            /*generic_associated_types workaround*/ unsafe {
                // compose shall not outlive Backend but cannot enforce without generic_associated_types
                // so this 'static lifetime workaround is used
                use std::mem;
                GLCommandProcessContext {
                    ll: mem::transmute(&mut self.ll),
                    index_store: mem::transmute(self.index_store.write()),
                    vertex_store: mem::transmute(self.vertex_store.write()),
                    texture_2d_store: mem::transmute(self.texture_2d_store.write()),
                    shader_program_store: mem::transmute(self.shader_program_store.write()),
                }
            }
        };

        context.index_store.finalize_requests();
        context.vertex_store.finalize_requests();
        context.texture_2d_store.finalize_requests();
        context.shader_program_store.finalize_requests();

        for cmd in consume.drain() {
            cmd.process(&mut context);
        }

        // release unreferenced resources
        let ref mut ll = self.ll;
        context.index_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        context.vertex_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        context.texture_2d_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
        context.shader_program_store.drain_unused_filtered(|data| {
            data.release(ll);
            true
        });
    }

    fn init_view(&self, viewport: Option<Viewport>, color: Option<Float32x4>, depth: Option<f32>)
    {
        self.get_queue().add_command(0,
                                     ClearCommand {
                                         color: color,
                                         depth: depth,
                                         viewport: viewport,
                                     });
    }

    fn get_view_size(&self) -> Size {
        self.get_screen_size()
    }

    fn get_view_aspect(&self) -> f32 {
        let size = self.get_screen_size();
        (size.width as f32) / (size.height as f32)
    }
}


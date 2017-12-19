use common::*;
use store::handlestore::*;

/// Trait to define the command.
pub trait Command {
    fn get_sort_key(&self) -> usize;
}

/// Trait to define the command buffer. This is the backend for each CommandQueue
pub trait CommandStore: Sync {
    /// Constructs a new CommandStore
    fn new() -> Self where Self: Sized;

    /// Add a command to the store
    fn add<C: Command>(&self, order: (ActivePassIndex, usize), cmd: C);
}

/// Trait to treat passes and render manager as a command que.
/// This is the frontend for CommandStore for easier use through Pass
pub trait CommandQueue {
    fn add<C: Command>(&self, cmd: C);
}

/// Trait for render resources
pub trait Resource {
    type Impl;

    /// Releases the allocated hw resources.
    fn release<Q: CommandQueue>(&self, queue: &Q);
}

pub trait ResourceManager {
    type CommandStore: CommandStore;

    type VertexBuffer: VertexBufferBase;
    type IndexBuffer: IndexBufferBase;
    type ShaderProgram: ShaderProgramBase;
    type RenderTarget: RenderTarget;
    type Texture2D: Texture2D;

    fn new() -> Self where Self: Sized;

    fn vertex_buffers(&self) -> Store<<Self::VertexBuffer as Resource>::Impl>;
    fn index_buffers(&self) -> Store<<Self::IndexBuffer as Resource>::Impl>;
    fn shader_programs(&self) -> Store<<Self::ShaderProgram as Resource>::Impl>;
    fn render_targets(&self) -> Store<<Self::RenderTarget as Resource>::Impl>;
    fn textures_2d(&self) -> Store<<Self::Texture2D as Resource>::Impl>;
}

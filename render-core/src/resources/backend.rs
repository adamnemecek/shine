use resources::*;
use types::*;

pub trait Backend: 'static {
    ///Trait to abstract the command queue.
    type CommandQueue;
    type CommandContext;
    type VertexBufferLayout;

    fn get_queue(&self) -> Self::CommandQueue;
    fn flush(&mut self);

    fn init_view(&self, viewport: Option<Viewport>, color: Option<Float32x4>, depth: Option<f32>);
    fn get_view_size(&self) -> Size;
    fn get_view_aspect(&self) -> f32;
}

use resources::*;
use types::*;

pub trait Backend: 'static {
    ///Trait to abstract the command queue.
    type CommandQueue;

    ///Trait to abstract low-level platform state changes.
    type CommandContext;

    fn get_queue(&self) -> Self::CommandQueue;
    fn flush(&mut self);

    fn init_view(&self, viewport: Option<Viewport>, color: Option<Float32x4>, depth: Option<f32>);
}

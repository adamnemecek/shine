use resources::*;
use types::*;

pub trait Backend: 'static {
    type CommandQueue;
    type CommandContext;
    type VertexBufferLayout;

    /// Returns the size of the render are on the associated window.
    fn get_screen_size(&self) -> Size;

    /// Returns the pixel aspect of the associated window.
    fn get_pixel_aspect(&self) -> f32;

    fn submit(&mut self, queue: &mut Self::CommandQueue);
    fn swap_buffers(&mut self);

    /// Sets up a view for rendering
    fn init_view(queue: &mut Self::CommandQueue, view: u8, viewport: Viewport, color: Option<Float32x4>, depth: Option<f32>);
}

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

    //todo: replace api for create_queue, submit_queue
    // window::start_update, start_render shall create/submit automatically
    fn get_queue(&self) -> Self::CommandQueue;
    fn flush(&mut self);
    fn swap_buffers(&mut self);

    /// Sets up a view for rendering
    fn init_view(&self, viewport: Viewport, color: Option<Float32x4>, depth: Option<f32>);
}

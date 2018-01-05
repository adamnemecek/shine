use core::*;
use framework::*;
use resources::*;


pub trait PlatformWindowBuilder {
    fn build<V: View<R=GLResources>>(&self, engine: &PlatformEngine, view: V) -> Result<PlatformWindow, Error>;
}


/// Window implementation for opengl
pub type PlatformWindow = Box<GLWindow>;

impl Window for Box<GLWindow> {
    type R = GLResources;

    fn close(&mut self) {
        if !self.is_closed() {
            self.as_mut().close()
        }
    }

    fn is_closed(&self) -> bool {
        self.as_ref().is_closed()
    }

    fn get_position(&self) -> Position {
        self.as_ref().get_position()
    }

    /// Gets the size of the window.
    fn get_size(&self) -> Size {
        self.as_ref().get_size()
    }

    /// Gets the size of the draw area of the window.
    fn get_draw_size(&self) -> Size {
        self.as_ref().get_draw_size()
    }

    /// Returns if the context of the window is ready for rendering
    fn is_ready_to_render(&self) -> bool {
        self.as_ref().is_ready_to_render()
    }

    /// Update view
    fn update_view(&mut self) {
        self.as_mut().update_view();
    }

    /// Triggers an immediate render.
    fn render(&mut self) -> Result<(), Error> {
        self.as_mut().render()
    }
}



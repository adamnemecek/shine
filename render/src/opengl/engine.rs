use std::rc::Rc;
use std::cell::RefCell;
use engine::*;
use opengl::*;

#[cfg(target_os = "windows")]
#[path = "platform/windows/engine.rs"]
mod platform_engine;

#[cfg(target_os = "windows")]
#[path = "platform/windows/window.rs"]
mod platform_window;

pub use self::platform_engine::GLEngine;
pub use self::platform_window::GLWindow;
pub use self::platform_window::win_messages;

/// Engine implementation for opengl
pub struct PlatformEngine {
    platform: Box<GLEngine>
}

impl PlatformEngine {
    /// Creates a new engine.
    pub fn new() -> Result<PlatformEngine, Error> {
        let platform = try!(GLEngine::new());
        Ok(PlatformEngine { platform: platform })
    }

    pub fn platform(&self) -> &GLEngine {
        self.platform.as_ref()
    }
}

impl Engine for PlatformEngine {
    fn quit(&self) {
        self.platform.quit();
    }

    fn dispatch_event(&self, timeout: DispatchTimeout) -> bool {
        self.platform.dispatch_event(timeout)
    }
}


/// Window settings with extra implementation for opengl
pub type PlatformWindowSettings = WindowSettings<GLWindowSettings>;

impl WindowSettings<GLWindowSettings> {
    /// Builds window from the given settings.
    ///
    /// # Errors
    ///
    /// This function will return an error if the backend cannot create the window
    pub fn build<V: View>(&self, engine: &PlatformEngine, view: V) -> Result<PlatformWindow, Error> {
        let view = Rc::new(RefCell::new(view));
        GLWindow::new_boxed(self, engine.platform(), view)
    }
}


/// Window implementation for opengl
pub type PlatformWindow = Box<GLWindow>;

impl Window for Box<GLWindow> {
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


pub struct PlatformResourceManager {}

impl ResourceManager for PlatformResourceManager {}


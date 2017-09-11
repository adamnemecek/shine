#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use render::*;


/// Enum to store the error occurred during a call to a render function.
#[derive(Debug, Clone)]
pub enum Error {
    /// Error reported during a window creation.
    CreationError(String),
    /// Error reported by the OS during rendering
    ContextError(String),
    /// Context is lost, ex window has been closed.
    ContextLost,
}


/// Callbacks for surface related event handling.
pub trait SurfaceEventHandler: 'static {
    /// Handles the surface lost event.
    ///
    /// Window still has the OS resources, but will be released soon after this call.
    fn on_lost(&mut self, &mut Window);

    /// Handles the surface ready event.
    ///
    /// Window has create all the OS resources.
    fn on_ready(&mut self, &mut Window);

    /// Handles the surface size or other config change.
    ///
    /// Window has create all the OS resources.
    fn on_changed(&mut self, &mut Window);
}

/// Callbacks for input related event handling.
pub trait InputEventHandler: 'static {
    /// Handles key down and up events.
    fn on_key(&mut self, &mut Window, sc: ScanCode, vk: Option<VirtualKeyCode>, is_down: bool);
}

/// Structure to store the window abstraction.
///
/// The structure stores the platform dependent implementation and serves as a bridge between
/// the abstraction and the concrete implementation.
pub struct Window {
    platform: Rc<RefCell<WindowImpl>>
}

impl Window {
    // Creates a new window with the given settings.
    ///
    /// # Error
    ///
    /// This function will return an error if the current backend cannot create the
    /// window.
    pub fn new(settings: WindowSettings, engine: &mut Engine) -> Result<Window, Error> {
        let platform = try!(WindowImpl::new(settings, engine));
        Ok(Window { platform: platform })
    }

    pub ( crate ) fn from_platform(platform: Rc<RefCell<WindowImpl>>) -> Window {
        Window { platform: platform }
    }

    /// Returns a reference to the platform specific implementation detail
    pub fn platform(&self) -> Ref<WindowImpl> {
        self.platform.borrow()
    }

    /// Returns a mutable reference to the platform specific implementation detail
    pub fn platform_mut(&mut self) -> RefMut<WindowImpl> {
        self.platform.borrow_mut()
    }

    /// Sets the surface event handler.
    ///
    /// Event handler can be altered any time, but it is preferred to set them before
    /// the show call no to miss the on_ready event.
    pub fn set_surface_handler<H: SurfaceEventHandler>(&mut self, handler: H) {
        self.platform.borrow_mut().set_surface_handler(handler);
    }

    /// Sets the input event handler.
    pub fn set_input_handler<H: InputEventHandler>(&mut self, handler: H) {
        self.platform.borrow_mut().set_input_handler(handler);
    }

    /// Starts the closing process.
    ///
    /// This function asks the OS to close the window. Window is not closed immediately,
    /// event handling shall continue the execution until the OS close events arrive.
    pub fn close(&mut self) {
        if self.is_closed() {
            return;
        }

        self.platform.borrow_mut().close()
    }

    /// Returns true if the window is closed or in closing state.
    pub fn is_closed(&self) -> bool {
        self.platform.borrow().is_closed()
    }

    /// Gets the position of the window.
    pub fn get_position(&self) -> Position {
        self.platform.borrow().get_position()
    }

    /// Gets the size of the window.
    pub fn get_size(&self) -> Size {
        self.platform.borrow().get_size()
    }

    /// Gets the size of the draw area of the window.
    pub fn get_draw_size(&self) -> Size {
        self.platform.borrow().get_draw_size()
    }

    /// Prepares the window for rendering.
    pub fn start_render(&self) -> Result<(), Error> {
        self.platform.borrow_mut().start_render()
    }

    /// Swaps the buffers and perform post render tasks.
    pub fn end_render(&self) -> Result<(), Error> {
        self.platform.borrow_mut().end_render()
    }
}

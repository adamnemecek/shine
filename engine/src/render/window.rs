#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::*;
use std::marker::PhantomData;
use render::*;

/// Implements the view dependent aspect of an application.
pub trait View: 'static {
    /// Handles the surface lost event.
    ///
    /// Window still has the OS resources, but will be released soon after this call.
    fn on_surface_lost(&mut self, window: &mut Window);

    /// Handles the surface ready event.
    ///
    /// Window has create all the OS resources.
    fn on_surface_ready(&mut self, window: &mut Window);

    /// Handles the surface size or other config change.
    ///
    /// Window has create all the OS resources.
    fn on_surface_changed(&mut self, window: &mut Window);

    /// Handles update requests.
    fn on_update(&mut self);

    /// Handles render requests.
    ///
    /// Rendering can be triggered manually by calling the render function of window or
    /// by the system if paint event handing is enabled.
    fn on_render(&mut self, window: &mut Window);

    /// Handles key down and up events.
    fn on_key(&mut self, window: &mut Window, scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool);
}


/// Trait for window abstraction.
pub trait Window<'engine> {
    /// Returns a reference to the platform specific implementation detail
    fn platform(&self) -> &WindowImpl;

    /// Returns a mutable reference to the platform specific implementation detail
    fn platform_mut(&mut self) -> &mut WindowImpl;

    /// Requests to close the window.
    ///
    /// This function asks the OS to close the window. Window is not closed immediately,
    /// event handling shall continue the execution until the OS close events arrive.
    fn close(&mut self) {
        if !self.is_closed() {
            self.platform_mut().close()
        }
    }

    /// Returns true if the window is closed or in closing state.
    fn is_closed(&self) -> bool {
        self.platform().is_closed()
    }

    /// Returns if the context of the window is ready for rendering
    fn is_read_to_render(&self) -> bool {
        self.platform().is_read_to_render()
    }

    /// Gets the position of the window.
    fn get_position(&self) -> Position {
        self.platform().get_position()
    }

    /// Gets the size of the window.
    fn get_size(&self) -> Size {
        self.platform().get_size()
    }

    /// Gets the size of the draw area of the window.
    fn get_draw_size(&self) -> Size {
        self.platform().get_draw_size()
    }
}

pub ( crate ) struct WindowData {
    pub ( crate ) view: Rc<RefCell<View>>,
    pub ( crate ) platform: WindowImpl,
}

/// Raw window for View type erasure
pub ( crate ) struct RawWindow<'tmp>(
    pub ( crate ) &'tmp mut WindowData,
);

impl<'engine> Window<'engine> for RawWindow<'engine> {
    fn platform(&self) -> &WindowImpl {
        &self.0.platform
    }

    fn platform_mut(&mut self) -> &mut WindowImpl {
        &mut self.0.platform
    }
}

/// Window with concrete view type
pub struct ViewWindow<'engine, V: View>(
    pub ( crate ) Box<WindowData>,
    pub ( crate ) PhantomData<(&'engine (), V)>
);

impl<'engine, V: View> Window<'engine> for ViewWindow<'engine, V> {
    fn platform(&self) -> &WindowImpl {
        &self.0.as_ref().platform
    }

    fn platform_mut(&mut self) -> &mut WindowImpl {
        &mut self.0.as_mut().platform
    }
}

impl<'engine, V: View> ViewWindow<'engine, V> {
    /// Create a new window with the given view
    pub fn new<'e>(settings: WindowSettings, engine: &'e Engine, view: V) -> Result<ViewWindow<'e, V>, Error> {
        WindowImpl::new(settings, engine, view)
    }

    /// Triggers an immediate update.
    pub fn update_view(&mut self) {
        self.0.view.borrow_mut().on_update();
    }

    /// Triggers an immediate render.
    pub fn render(&mut self) -> Result<(), Error> {
        if self.is_read_to_render() {
            try!(self.platform_mut().start_render());
            let view = self.0.view.clone();
            view.borrow_mut().on_render(self);
            try!(self.platform_mut().end_render());
        }
        Ok(())
    }
}

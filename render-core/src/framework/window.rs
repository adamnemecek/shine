#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use types::*;
use error::*;
use framework::*;

/// Implements the view dependent aspect of an application.
pub trait View: 'static {
    /// TBD.
    type R: 'static;

    /// Handles the surface lost event.
    ///
    /// Window still has the OS resources, but will be released soon after this call.
    fn on_surface_lost(&mut self, window: &mut Window<R=Self::R>);

    /// Handles the surface ready event.
    ///
    /// Window has create all the OS resources.
    fn on_surface_ready(&mut self, window: &mut Window<R=Self::R>);

    /// Handles the surface size or other config change.
    ///
    /// Window has create all the OS resources.
    fn on_surface_changed(&mut self, window: &mut Window<R=Self::R>);

    /// Handles update requests.
    fn on_update(&mut self);

    /// Handles render requests.
    ///
    /// Rendering can be triggered manually by calling the render function of window or
    /// by the system if paint event handing is enabled.
    fn on_render(&mut self, window: &mut Window<R=Self::R>);

    /// Handles key down and up events.
    fn on_key(&mut self, window: &mut Window<R=Self::R>, scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool);
}


/// Trait for window abstraction.
pub trait Window {
    /// TBD.
    type R: 'static;

    /// Requests to close the window.
    ///
    /// This function asks the OS to close the window. Window is not closed immediately,
    /// event handling shall continue the execution until the OS close events arrive.
    fn close(&mut self);

    /// Returns true if the window is closed or in closing state.
    fn is_closed(&self) -> bool;

    /// Gets the position of the window.
    fn get_position(&self) -> Position;

    /// Gets the size of the window.
    fn get_size(&self) -> Size;

    /// Gets the size of the draw area of the window.
    fn get_draw_size(&self) -> Size;

    /// Returns if the context of the window is ready for rendering
    fn is_ready_to_render(&self) -> bool;

    /// Update view
    fn update_view(&mut self);

    /// Triggers an immediate render.
    fn render(&mut self) -> Result<(), Error>;
}

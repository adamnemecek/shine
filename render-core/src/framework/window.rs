#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use types::*;
use error::*;
use framework::*;

/// Trait for window abstraction.
pub trait Window<E: Engine>: Send {
    /// Requests to close the window.
    fn close(&mut self);

    /// Returns the position of the window.
    fn get_position(&self) -> Position;

    /// Returns the decorated size of the window.
    fn get_size(&self) -> Size;

    /// Returns the size of the render area (size of the screen without decoration).
    fn get_draw_size(&self) -> Size;

    /// Returns the backend (render context).
    fn backend(&mut self) -> &mut E::Backend;
}


/// Trait to manage the lifetime of a window
pub trait WindowHandler<E: Engine> {
    /// Return if window is closed
    fn is_closed(&self) -> bool;

    /// Request to close the window
    fn close(&mut self);

    /// Send a custom window command
    fn send_command(&self, cmd: WindowCommand);

    /// Send a custom window command and wait for it's result
    fn send_sync_command(&self, cmd: WindowCommand);
}


/// Trait to build windows. It is usually implemented by a concrete PlatformWindowSettings
pub trait WindowBuilder<E: Engine> {
    /// The window handler type
    type WindowHandler: WindowHandler<E>;

    /// The native window type
    type Window: Window<E>;

    /// Builds a window and start the associated render thread.
    fn build<Ctx, F>(&self, engine: &E, timeout: DispatchTimeout, ctx: Ctx, callback: F) -> Result<Self::WindowHandler, Error>
        where
            F: 'static + Send + Fn(&mut Self::Window, &mut Ctx, &WindowCommand),
            Ctx: 'static + Send;
}


#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::ops::{Deref, DerefMut};
use types::*;
use error::*;
use framework::*;
use resources::*;


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

    /// Returns a scope for general hardware resource update.
    fn start_update<'a>(&'a mut self) -> Option<RefUpdate<'a, E>>;

    /// Returns a scope for rendering
    fn start_render<'a>(&'a mut self) -> Option<RefRender<'a, E>>;
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


/// Guard for hardware resource update.
/// See Window::start_update.
pub struct RefUpdate<'a, E: Engine> {
    backend: &'a mut E::Backend,
}

impl<'a, E: Engine> RefUpdate<'a, E> {
    /// Constructs a RefUpdate.
    pub fn new<'b>(backend: &'b mut E::Backend) -> RefUpdate<'b, E> {
        RefUpdate {
            backend: backend
        }
    }
}

impl<'a, E: Engine> Deref for RefUpdate<'a, E> {
    type Target = E::Backend;

    fn deref(&self) -> &E::Backend {
        self.backend
    }
}

impl<'a, E: Engine> DerefMut for RefUpdate<'a, E> {
    fn deref_mut(&mut self) -> &mut E::Backend {
        self.backend
    }
}

impl<'a, E: Engine> Drop for RefUpdate<'a, E> {
    fn drop(&mut self) {
        self.backend.flush();
    }
}


/// Guard for rendering.
/// See Window::start_render.
pub struct RefRender<'a, E: Engine> {
    backend: &'a mut E::Backend,
}

impl<'a, E: Engine> RefRender<'a, E> {
    /// Constructs a RefRender.
    pub fn new<'b>(backend: &'b mut E::Backend) -> RefRender<'b, E> {
        RefRender {
            backend: backend
        }
    }
}

impl<'a, E: Engine> Deref for RefRender<'a, E> {
    type Target = E::Backend;

    fn deref(&self) -> &E::Backend {
        self.backend
    }
}

impl<'a, E: Engine> DerefMut for RefRender<'a, E> {
    fn deref_mut(&mut self) -> &mut E::Backend {
        self.backend
    }
}

impl<'a, E: Engine> Drop for RefRender<'a, E> {
    fn drop(&mut self) {
        self.backend.flush();
        self.backend.swap_buffers();
    }
}

#![cfg(any(target_os = "windows"))]

use render::winapi;

use render::*;

pub struct Context {}

impl Context {
    /// Creates a Wgl context with the given config.
    ///
    /// # Error
    /// If context connat be created an error is returned describing the reason.
    pub fn new(_: winapi::HINSTANCE, _: winapi::HWND, _: &WindowSettings) -> Result<Context, Error> {
        Err(Error::CreationError(format!("EGL context is not supported yet")))
    }

    /// Makes this context active.
    #[inline]
    pub fn make_current(&self) -> Result<(), Error> {
        Ok(())
    }

    /// Swaps the back and front buffers
    #[inline]
    pub fn swap_buffers(&self) -> Result<(), Error> {
        Ok(())
    }
}

#![cfg(any(target_os = "windows"))]

use winapi;

use engine::*;
use opengl::*;


/// Egl context
pub struct Context {}

impl Context {
    /// Creates an Egl context with the given config.
    ///
    /// # Error
    /// If context connat be created an error is returned describing the reason.
    pub fn new(_: winapi::HINSTANCE, _: winapi::HWND, _: &PlatformWindowSettings) -> Result<Context, Error> {
        Err(Error::WindowCreationError(format!("EGL context is not supported yet")))
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

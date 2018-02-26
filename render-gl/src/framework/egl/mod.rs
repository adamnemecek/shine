#![cfg(any(target_os = "windows"))]

use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use core::*;
use framework::*;


/// Egl context
pub struct Context {}

impl Context {
    /// Creates an Egl context with the given config.
    ///
    /// # Error
    /// If context connat be created an error is returned describing the reason.
    pub fn new(_: HINSTANCE, _: HWND, _: &PlatformWindowSettings) -> Result<Context, Error> {
        Err(Error::WindowCreationError(format!("EGL context is not supported yet")))
    }

    /// Makes this context active.
    #[inline]
    pub fn activate(&self) -> Result<(), Error> {
        Ok(())
    }

    /// Makes this context inactive.
    #[inline]
    pub fn deactivate(&self) -> Result<(), Error> {
        Ok(())
    }

    /// Swaps the back and front buffers
    #[inline]
    pub fn swap_buffers(&self) -> Result<(), Error> {
        Ok(())
    }
}

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use core::*;
use framework::*;

/// Enum defining the context for the render backend
pub enum GLContext {
    EGLContext(egl::Context),
    WGLContext(wgl::Context)
}


/// Context implementation for Windows.
///
/// It support the EGL and WGL API interfaces. The implementation can be selected
/// through WindowSettings.
impl GLContext {
    pub fn new(app_instance: HINSTANCE, hwnd: HWND, settings: &PlatformWindowSettings) -> Result<GLContext, Error> {
        match settings.platform_extra.gl_profile {
            OpenGLProfile::ES2 => {
                // for opengl es2 we are using egl context
                let ctx = try!(egl::Context::new(app_instance, hwnd, settings));
                Ok(GLContext::EGLContext(ctx))
            }

            _ => {
                // and wgl for everything else
                let ctx = try!(wgl::Context::new(app_instance, hwnd, settings));
                Ok(GLContext::WGLContext(ctx))
            }
        }
    }

    pub fn activate(&self) -> Result<(), Error> {
        match self {
            &GLContext::EGLContext(ref ctx) => { ctx.activate() }
            &GLContext::WGLContext(ref ctx) => { ctx.activate() }
        }
    }

    pub fn deactivate(&self) -> Result<(), Error> {
        match self {
            &GLContext::EGLContext(ref ctx) => { ctx.deactivate() }
            &GLContext::WGLContext(ref ctx) => { ctx.deactivate() }
        }
    }

    pub fn swap_buffers(&self) -> Result<(), Error> {
        match self {
            &GLContext::EGLContext(ref ctx) => { ctx.swap_buffers() }
            &GLContext::WGLContext(ref ctx) => { ctx.swap_buffers() }
        }
    }
}

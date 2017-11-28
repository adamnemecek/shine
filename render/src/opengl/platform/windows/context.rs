use backend::opengl::context::wgl;
use backend::opengl::context::egl;

use winapi;
use backend::*;


/// Enum defining the context for the render backend
pub enum Context {
    EGLContext(egl::Context),
    WGLContext(wgl::Context)
}


/// Context implementation for Windows.
///
/// It support the EGL and WGL API interfaces. The implementation can be selected
/// through WindowSettings.
impl Context {
    pub fn new(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND, settings: &WindowSettings) -> Result<Context, Error> {
        match settings.fb_config.gl_profile {
            OpenGLProfile::ES2 => {
                // for opengl es2 we are using egl context
                let ctx = try!(egl::Context::new(app_instance, hwnd, settings));
                Ok(Context::EGLContext(ctx))
            }

            _ => {
                // and wgl for everything else
                let ctx = try!(wgl::Context::new(app_instance, hwnd, settings));
                Ok(Context::WGLContext(ctx))
            }
        }
    }

    pub fn make_current(&self) -> Result<(), Error> {
        match self {
            &Context::EGLContext(ref ctx) => { ctx.make_current() }
            &Context::WGLContext(ref ctx) => { ctx.make_current() }
        }
    }

    pub fn swap_buffers(&self) -> Result<(), Error> {
        match self {
            &Context::EGLContext(ref ctx) => { ctx.swap_buffers() }
            &Context::WGLContext(ref ctx) => { ctx.swap_buffers() }
        }
    }
}
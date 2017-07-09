use render::opengl::context::wgl;
use render::opengl::context::egl;

use render::*;
use render::winapi;

pub enum Context {
    EGLContext(egl::Context),
    WGLContext(wgl::Context)
}

impl Context {
    pub fn new_wgl(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND, settings: &WindowSettings) -> Result<Context, Error> {
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
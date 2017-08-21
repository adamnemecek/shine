#![cfg(any(target_os = "windows"))]

/// WGL bindings
pub mod wgl {
    include!(concat!(env!("OUT_DIR"), "/wgl_bindings.rs"));
}

/// Functions that are not necessarly always available
pub mod wgl_extra {
    include!(concat!(env!("OUT_DIR"), "/wgl_extra_bindings.rs"));
}

#[link(name = "opengl32")]
extern {}

mod dummywindow;

use std::ptr;
use std::io;
use std::mem;
use std::ffi::{CStr, CString, OsStr};
use std::os::raw::{c_void, c_int};
use std::os::windows::ffi::OsStrExt;

use render::user32;
use render::winapi;

use render::*;
use self::dummywindow::*;


// Loads the gl library as we have static linking, sry.
unsafe fn load_gl_library() -> Result<winapi::HMODULE, CreationError> {
    let name = OsStr::new("opengl32.dll").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

    let lib = kernel32::LoadLibraryW(name.as_ptr());
    if lib.is_null() {
        return Err(CreationError::OsError(format!("LoadLibrary function failed: {}", io::Error::last_os_error())));
    }

    Ok(lib)
}

unsafe fn load_wgl_extension(hdc: winapi::HDC) -> Result<wgl_extra::Wgl, CreationError> {
    let mut pfd: winapi::PIXELFORMATDESCRIPTOR = mem::zeroed();
    pfd.nVersion = 1;
    pfd.dwFlags = winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER;
    pfd.iPixelType = winapi::PFD_TYPE_RGBA;
    pfd.cColorBits = 24;

    let pixel_format = gdi32::ChoosePixelFormat(hdc, &pfd);
    if gdi32::SetPixelFormat(hdc, pixel_format, &pfd) != winapi::TRUE {
        return Err(CreationError::OsError(format!("WGL: Failed to set pixel format for dummy context: {}", io::Error::last_os_error())));
    }

    let rc = wgl::CreateContext(hdc as *const c_void);
    if rc.is_null() {
        return Err(CreationError::OsError(format!("WGL: Failed to create dummy context: {}", io::Error::last_os_error())));
    }

    if wgl::MakeCurrent(hdc as *const c_void, rc) != winapi::TRUE {
        wgl::DeleteContext(rc);
        return Err(CreationError::OsError(format!("WGL: Failed to make dummy context current: {}", io::Error::last_os_error())));
    }

    let wgl_extra = wgl_extra::Wgl::load_with(|addr| {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();
        wgl::GetProcAddress(addr) as *const c_void
    });

    wgl::MakeCurrent(hdc as *const c_void, ptr::null_mut());
    wgl::DeleteContext(rc);
    Ok(wgl_extra)
}

pub struct Context {
    /// loaded gl library
    gl_library: winapi::HMODULE,
    /// wgl extensions
    wgl_ext: wgl_extra::Wgl,
    // gl extensions
    //gl_ext: gl_extra::gl,
}

impl Context {
    pub fn new(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND, hdc: winapi::HDC) -> Result<Context, CreationError> {
        unsafe {
            let hdc = user32::GetDC(hwnd);

            let gl_library = try!(load_gl_library());
            let wgl_ext = try!(load_wgl_extension(hdc));

            // getting the list of the supported extensions
            let extensions = if wgl_ext.GetExtensionsStringARB.is_loaded() {
                let data = wgl_ext.GetExtensionsStringARB(hdc as *const _);
                let data = CStr::from_ptr(data).to_bytes().to_vec();
                String::from_utf8(data).unwrap()
            } else if wgl_ext.GetExtensionsStringEXT.is_loaded() {
                let data = wgl_ext.GetExtensionsStringEXT();
                let data = CStr::from_ptr(data).to_bytes().to_vec();
                String::from_utf8(data).unwrap()
            } else {
                format!("")
            };
            println!("wgl extensions: {}", extensions);


            Ok(Context {
                gl_library: gl_library,
                wgl_ext: wgl_ext,
                //gl_ext: gl_ext,
            })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            kernel32::FreeLibrary(self.gl_library);
        }
    }
}

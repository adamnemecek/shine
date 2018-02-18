#![cfg(any(target_os = "windows"))]

use framework::*;

/// WGL bindings
pub mod wgl {
    include!(concat!(env!("OUT_DIR"), "/wgl_bindings.rs"));
}

/// Functions that are not necessarily always available
pub mod wgl_ext {
    include!(concat!(env!("OUT_DIR"), "/wgl_ext_bindings.rs"));
}

#[link(name = "opengl32")]
extern {}

mod dummywindow;

use std::ptr;
use std::io;
use std::i32;
use std::mem;
use std::ffi::{CStr, CString, OsStr};
use std::os::raw::{c_void, c_int};
use std::os::windows::ffi::OsStrExt;

use winapi;
use kernel32;
use gdi32;
use user32;

use core::*;
use lowlevel::*;
use self::dummywindow::DummyWindow;

// Loads the gl library
fn load_gl_library() -> Result<winapi::HMODULE, Error> {
    let dll_name = "opengl32.dll";
    let name = OsStr::new(dll_name).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

    let lib = ffi!(kernel32::LoadLibraryW(name.as_ptr()));
    if lib.is_null() {
        return Err(Error::WindowCreationError(format!("WGL: LoadLibrary function failed for {}: {}", dll_name, io::Error::last_os_error())));
    }

    Ok(lib)
}

// Loads the wgl extensions
fn load_wgl_extension(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND) -> Result<(wgl_ext::Wgl, String), Error> {
    let dummy_window = try!(DummyWindow::new(app_instance, hwnd));
    let hdc = dummy_window.hdc;

    let mut pfd: winapi::PIXELFORMATDESCRIPTOR = unsafe { mem::zeroed() };
    pfd.nVersion = 1;
    pfd.dwFlags = winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER;
    pfd.iPixelType = winapi::PFD_TYPE_RGBA;
    pfd.cColorBits = 24;

    let pixel_format = ffi!(gdi32::ChoosePixelFormat(hdc, &pfd));
    if ffi!(gdi32::SetPixelFormat(hdc, pixel_format, &pfd)) != winapi::TRUE {
        return Err(Error::WindowCreationError(format!("WGL: Failed to set pixel format for dummy context: {}", io::Error::last_os_error())));
    }

    let rc = ffi!(wgl::CreateContext(hdc as *const c_void));
    if rc.is_null() {
        return Err(Error::WindowCreationError(format!("WGL: Failed to create dummy context: {}", io::Error::last_os_error())));
    }

    if ffi!(wgl::MakeCurrent(hdc as *const c_void, rc)) != winapi::TRUE {
        ffi!(wgl::DeleteContext(rc));
        return Err(Error::WindowCreationError(format!("WGL: Failed to make dummy context current: {}", io::Error::last_os_error())));
    }

    let wgl_ext = ffi!(wgl_ext::Wgl::load_with(|addr| {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();
        wgl::GetProcAddress(addr) as *const c_void
    }));

    let wgl_extensions = get_wgl_extension_string(&wgl_ext, hdc);

    ffi!(wgl::MakeCurrent(hdc as *const c_void, ptr::null_mut()));
    ffi!(wgl::DeleteContext(rc));
    Ok((wgl_ext, wgl_extensions))
}

// Gets the list of the supported extensions
fn get_wgl_extension_string(wgl_ext: &wgl_ext::Wgl, hdc: winapi::HDC) -> String {
    if wgl_ext.GetExtensionsStringARB.is_loaded() {
        let data = ffi!(wgl_ext.GetExtensionsStringARB(hdc as *const _));
        let data = unsafe { CStr::from_ptr(data) }.to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    } else if wgl_ext.GetExtensionsStringEXT.is_loaded() {
        let data = ffi!(wgl_ext.GetExtensionsStringEXT());
        let data = unsafe { CStr::from_ptr(data) }.to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    } else {
        "".to_string()
    }
}


/// Structure to handle WGL context
pub struct Context {
    pixel_format_id: u32,
    hwnd: winapi::HWND,
    hdc: winapi::HDC,
    hglrc: winapi::HGLRC,

    gl_library: winapi::HMODULE,
    wgl_ext: wgl_ext::Wgl,
    wgl_extensions: String,
}

impl Context {
    /// Creates a Wgl context with the given config.
    ///
    /// # Error
    /// If context cannot be created an error is returned describing the reason.
    pub fn new(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND, settings: &PlatformWindowSettings) -> Result<Context, Error> {
        let gl_library = try!(load_gl_library());
        let (wgl_ext, wgl_extensions) = try!(load_wgl_extension(app_instance, hwnd));

        //println!("wgl extensions: {}", wgl_extensions);
        let mut context = Context {
            pixel_format_id: 0,
            hwnd: hwnd,
            hdc: 0 as winapi::HDC,
            hglrc: 0 as winapi::HGLRC,

            gl_library: gl_library,
            wgl_ext: wgl_ext,
            wgl_extensions: wgl_extensions,
        };

        // create dc
        context.hdc = ffi!(user32::GetDC(hwnd));
        if context.hdc.is_null() {
            return Err(Error::WindowCreationError(format!("WGL: Failed to get dc: {}", io::Error::last_os_error())));
        }

        // find a matching pixel foramt
        context.pixel_format_id = try!(context.choose_pixel_format(&settings.fb_config));
        if context.pixel_format_id == 0 {
            return Err(Error::WindowCreationError("WGL: Failed to find a suitable pixel format".to_string()));
        }

        // create context
        context.hglrc = try!(context.create_context(context.pixel_format_id, &settings.fb_config, &settings.platform_extra));

        try!(context.make_current());
        try!(context.load_gl_functions());

        Ok(context)
    }

    /// Makes this context active.
    #[inline]
    pub fn make_current(&self) -> Result<(), Error> {
        assert!(!self.hglrc.is_null());
        if ffi!( wgl::MakeCurrent(self.hdc as *const _, self.hglrc as *const _)) != 0 {
            Ok(())
        } else {
            Err(Error::ContextError(format!("Make current failed: {}", io::Error::last_os_error())))
        }
    }

    /// Swaps the back and front buffers
    #[inline]
    pub fn swap_buffers(&self) -> Result<(), Error> {
        if ffi!(gdi32::SwapBuffers(self.hdc)) != 0 {
            Ok(())
        } else {
            Err(Error::ContextError(format!("Swap buffers failed: {}", io::Error::last_os_error())))
        }
    }

    /// Returns if the given extension is supprted.
    pub fn has_wgl_extension(&self, extension: &str) -> bool {
        self.wgl_extensions.split(' ').find(|&i| i == extension).is_some()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const winapi::c_void {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();

        let p = ffi!(wgl::GetProcAddress(addr) as *const winapi::c_void);
        if !p.is_null() { return p; }
        //let p = ffi!(gl::GetProcAddress(addr) as *const winapi::c_void);
        //if !p.is_null() { return p; }
        ffi!(kernel32::GetProcAddress(self.gl_library, addr)) as *const winapi::c_void
    }

    pub fn get_pixel_format_config(&self) -> Result<FBConfig, Error> {
        self.get_pixel_format_info(self.pixel_format_id)
    }

    /// Gets an attribute of a pixel format using the "modern" extension
    fn get_pixel_format_attrib(&self, pixel_format_id: u32, attrib: u32) -> Result<u32, Error> {
        let mut value: c_int = 0;
        assert!(self.has_wgl_extension("WGL_ARB_pixel_format"));
        if ffi!(self.wgl_ext.GetPixelFormatAttribivARB(self.hdc as *const _, pixel_format_id as c_int, 0, 1, [attrib as c_int].as_ptr(), &mut value)) == 0 {
            return Err(Error::WindowCreationError(format!("WGL: Failed to retrieve pixel format attribute n:{}, attrib:{}", pixel_format_id, attrib)));
        }

        Ok(value as u32)
    }

    /// Gets the pixel format info for the given id using the "modern" extensions
    ///
    /// # Error
    /// If some attribute is not matching or cannot be queried, an error is returned.
    fn get_pixel_format_info_ext(&self, n: u32) -> Result<FBConfig, Error> {
        // Get pixel format attributes through "modern" extension
        if try!(self.get_pixel_format_attrib(n, wgl_ext::SUPPORT_OPENGL_ARB)) != 1 ||
            try!(self.get_pixel_format_attrib(n, wgl_ext::DRAW_TO_WINDOW_ARB)) != 1 {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: Not OpenGL compatible", n)));
        }

        if try!(self.get_pixel_format_attrib(n, wgl_ext::PIXEL_TYPE_ARB)) != wgl_ext::TYPE_RGBA_ARB {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: Not RGBA", n)));
        }

        if try!(self.get_pixel_format_attrib(n, wgl_ext::ACCELERATION_ARB)) == wgl_ext::NO_ACCELERATION_ARB {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: No hardware acceleration", n)));
        }

        Ok(FBConfig {
            handle: n,
            red_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::RED_BITS_ARB)) as u8,
            green_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::GREEN_BITS_ARB)) as u8,
            blue_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::BLUE_BITS_ARB)) as u8,
            alpha_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::ALPHA_BITS_ARB)) as u8,
            depth_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::DEPTH_BITS_ARB)) as u8,
            stencil_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::STENCIL_BITS_ARB)) as u8,

            accum_red_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::ACCUM_RED_BITS_ARB)) as u8,
            accum_green_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::ACCUM_GREEN_BITS_ARB)) as u8,
            accum_blue_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::ACCUM_BLUE_BITS_ARB)) as u8,
            accum_alpha_bits: try!(self.get_pixel_format_attrib(n, wgl_ext::ACCUM_ALPHA_BITS_ARB)) as u8,
            aux_buffers: try!(self.get_pixel_format_attrib(n, wgl_ext::AUX_BUFFERS_ARB)) as u8,

            samples: if self.has_wgl_extension("WGL_ARB_multisample") { try!(self.get_pixel_format_attrib(n, wgl_ext::SAMPLES_ARB)) as u8 } else { 0 },
            stereo: try!(self.get_pixel_format_attrib(n, wgl_ext::STEREO_ARB)) == 1,
            double_buffer: try!(self.get_pixel_format_attrib(n, wgl_ext::DOUBLE_BUFFER_ARB)) == 1,
            srgb: if self.has_wgl_extension("WGL_ARB_framebuffer_sRGB") || self.has_wgl_extension("WGL_EXT_framebuffer_sRGB") {
                try!(self.get_pixel_format_attrib(n, wgl_ext::FRAMEBUFFER_SRGB_CAPABLE_ARB)) == 1
                //} else if self.has_extension("WGL_EXT_colorspace") {
                //    try!(self.get_pixel_format_attrib(n, wgl_ext::COLORSPACE_EXT)) == wgl_ext::COLORSPACE_SRGB_EXT
            } else { false },

            // values not considered or not part of pixel format
            debug: false,
            vsync: false,
        })
    }

    /// Gets the pixel format info for the given id using the legacy pfb method
    ///
    /// # Error
    /// If some attribute is not matching or cannot be queried, an error is returned.
    fn get_pixel_format_info_pfd(&self, n: u32) -> Result<FBConfig, Error> {
        let mut pfd: winapi::PIXELFORMATDESCRIPTOR = unsafe { mem::zeroed() };

        if ffi!(gdi32::DescribePixelFormat(self.hdc, n as c_int, mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u32, &mut pfd)) == 0 {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: DescribePixelFormat failed", n)));
        }

        if (pfd.dwFlags & winapi::PFD_DRAW_TO_WINDOW) == 0 || (pfd.dwFlags & winapi::PFD_SUPPORT_OPENGL) == 0 {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: Not OpenGL compatible", n)));
        }

        if (pfd.dwFlags & winapi::PFD_GENERIC_ACCELERATED) == 0 && (pfd.dwFlags & winapi::PFD_GENERIC_FORMAT) != 0 {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: No hardware acceleration", n)));
        }

        if pfd.iPixelType != winapi::PFD_TYPE_RGBA {
            return Err(Error::WindowCreationError(format!("WGL: query pixel format for {}: Not RGBA", n)));
        }

        Ok(FBConfig {
            handle: n,
            red_bits: pfd.cRedBits,
            green_bits: pfd.cGreenBits,
            blue_bits: pfd.cBlueBits,
            alpha_bits: pfd.cAlphaBits,
            depth_bits: pfd.cDepthBits,
            stencil_bits: pfd.cStencilBits,

            accum_red_bits: pfd.cAccumRedBits,
            accum_green_bits: pfd.cAccumGreenBits,
            accum_blue_bits: pfd.cAccumBlueBits,
            accum_alpha_bits: pfd.cAccumAlphaBits,
            aux_buffers: pfd.cAuxBuffers,

            samples: 0,
            stereo: (pfd.dwFlags & winapi::PFD_STEREO) != 0,
            double_buffer: (pfd.dwFlags & winapi::PFD_DOUBLEBUFFER) != 0,
            srgb: false,

            // values not considered or not part of pixel format
            debug: false,
            vsync: false,
        })
    }

    fn get_pixel_format_info(&self, n: u32) -> Result<FBConfig, Error> {
        if self.has_wgl_extension("WGL_ARB_pixel_format") {
            self.get_pixel_format_info_ext(n)
        } else {
            self.get_pixel_format_info_pfd(n)
        }
    }

    /// Gets tha available pixel formats using the "modern" extension
    fn get_pixel_formats_ext(&self) -> Result<Vec<FBConfig>, Error> {
        assert!(self.has_wgl_extension("WGL_ARB_pixel_format"));

        let native_count = try!(self.get_pixel_format_attrib(1, wgl_ext::NUMBER_PIXEL_FORMATS_ARB));
        let mut formats: Vec<FBConfig> = vec!();
        formats.reserve(native_count as usize);

        for i in 0..native_count {
            let n = i + 1;
            match self.get_pixel_format_info_ext(n) {
                Ok(u) => formats.push(u),
                //Err(e) => { println!("Pixel format dropped (extension): {}", e.0) }
                _ => {}
            };
        }

        Ok(formats)
    }

    /// Gets tha available pixel formats using the legacy PFDs
    fn get_pixel_formats_pfd(&self) -> Result<Vec<FBConfig>, Error> {
        let native_count = ffi!(gdi32::DescribePixelFormat(self.hdc, 1, mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u32, ptr::null_mut())) as u32;
        let mut formats: Vec<FBConfig> = vec!();
        formats.reserve(native_count as usize);

        for i in 0..native_count {
            let n = i + 1;
            match self.get_pixel_format_info_pfd(n) {
                Ok(u) => formats.push(u),
                //Err(e) => { println!("Pixel format dropped (pfd): {}", e.0) }
                _ => {}
            };
        }

        Ok(formats)
    }

    fn get_pixel_formats(&self) -> Result<Vec<FBConfig>, Error> {
        if self.has_wgl_extension("WGL_ARB_pixel_format") {
            self.get_pixel_formats_ext()
        } else {
            self.get_pixel_formats_pfd()
        }
    }

    /// Finds a best matching pixel form for the given config
    fn choose_pixel_format(&self, desired: &FBConfig) -> Result<u32, Error> {
        let configs = try!(self.get_pixel_formats());

        if configs.is_empty() {
            return Err(Error::WindowCreationError("WGL: The driver does not appear to support OpenGL".to_string()));
        }

        let mut least_missing = i32::MAX;
        let mut least_color_diff = i32::MAX;
        let mut least_extra_diff = i32::MAX;
        let mut closest_handle: u32 = 0; // 0 is an invalid pixel_format

        for current in configs.iter() {
            if desired.stereo != current.stereo {
                // Stereo is a hard constraint
                continue;
            }

            if desired.double_buffer != current.double_buffer {
                // Double buffering is a hard constraint
                continue;
            }

            let mut missing: i32 = 0;
            {
                // Count number of missing buffers
                if desired.alpha_bits > 0 && current.alpha_bits == 0 { missing += 1; }
                if desired.depth_bits > 0 && current.depth_bits == 0 { missing += 1; }
                if desired.stencil_bits > 0 && current.stencil_bits == 0 { missing += 1; }

                if desired.aux_buffers > 0 && current.aux_buffers < desired.aux_buffers {
                    missing += desired.aux_buffers as i32 - current.aux_buffers as i32;
                }

                if desired.samples > 0 && current.samples == 0 {
                    // Technically, several multisampling buffers could be
                    // involved, but that's a lower level implementation detail and
                    // not important to us here, so we count them as one
                    missing += 1;
                }
            }

            // These polynomials make many small channel size differences matter
            // less than one large channel size difference

            let mut color_diff: i32 = 0;
            {
                // Calculate color channel size difference value

                if desired.red_bits != FBCONFIG_DONT_CARE {
                    color_diff += (desired.red_bits as i32 - current.red_bits as i32) * (desired.red_bits as i32 - current.red_bits as i32);
                }

                if desired.green_bits != FBCONFIG_DONT_CARE {
                    color_diff += (desired.green_bits as i32 - current.green_bits as i32) * (desired.green_bits as i32 - current.green_bits as i32);
                }

                if desired.blue_bits != FBCONFIG_DONT_CARE {
                    color_diff += (desired.blue_bits as i32 - current.blue_bits as i32) * (desired.blue_bits as i32 - current.blue_bits as i32);
                }
            }


            let mut extra_diff: i32 = 0;
            {
                // Calculate non-color channel size difference value
                if desired.alpha_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.alpha_bits as i32 - current.alpha_bits as i32) * (desired.alpha_bits as i32 - current.alpha_bits as i32);
                }

                if desired.depth_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.depth_bits as i32 - current.depth_bits as i32) * (desired.depth_bits as i32 - current.depth_bits as i32);
                }

                if desired.stencil_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.stencil_bits as i32 - current.stencil_bits as i32) * (desired.stencil_bits as i32 - current.stencil_bits as i32);
                }

                if desired.accum_red_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.accum_red_bits as i32 - current.accum_red_bits as i32) * (desired.accum_red_bits as i32 - current.accum_red_bits as i32);
                }

                if desired.accum_green_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.accum_green_bits as i32 - current.accum_green_bits as i32) * (desired.accum_green_bits as i32 - current.accum_green_bits as i32);
                }

                if desired.accum_blue_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.accum_blue_bits as i32 - current.accum_blue_bits as i32) * (desired.accum_blue_bits as i32 - current.accum_blue_bits as i32);
                }

                if desired.accum_alpha_bits != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.accum_alpha_bits as i32 - current.accum_alpha_bits as i32) * (desired.accum_alpha_bits as i32 - current.accum_alpha_bits as i32);
                }

                if desired.samples != FBCONFIG_DONT_CARE {
                    extra_diff += (desired.samples as i32 - current.samples as i32) * (desired.samples as i32 - current.samples as i32);
                }

                if desired.srgb && !current.srgb { extra_diff += 1; }
            }

            // Figure out if the current one is better than the best one found so far
            // Least number of missing buffers is the most important heuristic,
            // then color buffer size match and lastly size match for other buffers

            if missing < least_missing {
                closest_handle = current.handle;
            } else if missing == least_missing {
                if (color_diff < least_color_diff) ||
                    (color_diff == least_color_diff && extra_diff < least_extra_diff) {
                    closest_handle = current.handle;
                }
            }

            if current.handle == closest_handle {
                least_missing = missing;
                least_color_diff = color_diff;
                least_extra_diff = extra_diff;
            }
        }

        Ok(closest_handle)
    }

    fn create_context(&self, pixel_format: u32, config: &FBConfig, extra: &GLExtraWindowSettings) -> Result<(winapi::HGLRC), Error> {
        let share: winapi::HGLRC = ptr::null_mut(); // no sharing is implemented yet

        let mut pfd: winapi::PIXELFORMATDESCRIPTOR = unsafe { mem::zeroed() };

        if ffi!(gdi32::DescribePixelFormat(self.hdc, pixel_format as c_int, mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u32, &mut pfd)) == 0 {
            return Err(Error::WindowCreationError(format!("WGL: Failed to retrieve PFD for selected pixel format ({}): {}", pixel_format, io::Error::last_os_error())));
        }

        if ffi!(gdi32::SetPixelFormat(self.hdc, pixel_format as c_int, &pfd)) == 0 {
            return Err(Error::WindowCreationError(format!("WGL: Failed to set selected pixel format ({}): {}", pixel_format, io::Error::last_os_error())));
        }

        if extra.gl_forward_compatible && !self.has_wgl_extension("WGL_ARB_create_context") {
            return Err(Error::WindowCreationError(format!("WGL: A forward compatible OpenGL context requested but WGL_ARB_create_context is unavailable")));
        }

        if extra.gl_profile != OpenGLProfile::DontCare && !self.has_wgl_extension("WGL_ARB_create_context_profile") {
            return Err(Error::WindowCreationError(format!("WGL: OpenGL profile requested but WGL_ARB_create_context_profile is unavailable")));
        }

        // collect attributes
        if self.has_wgl_extension("WGL_ARB_create_context") {
            let mut attribs: Vec<u32> = vec!();
            let mut flags: u32 = 0;

            if extra.gl_profile != OpenGLProfile::DontCare {
                if extra.gl_profile == OpenGLProfile::ES2 {
                    attribs.push(wgl_ext::CONTEXT_PROFILE_MASK_ARB);
                    attribs.push(wgl_ext::CONTEXT_ES2_PROFILE_BIT_EXT);
                } else {
                    if extra.gl_forward_compatible {
                        flags = flags | wgl_ext::CONTEXT_FORWARD_COMPATIBLE_BIT_ARB;
                    }

                    if extra.gl_profile == OpenGLProfile::Core {
                        attribs.push(wgl_ext::CONTEXT_PROFILE_MASK_ARB);
                        attribs.push(wgl_ext::CONTEXT_CORE_PROFILE_BIT_ARB);
                    } else if extra.gl_profile == OpenGLProfile::Compatibility {
                        attribs.push(wgl_ext::CONTEXT_PROFILE_MASK_ARB);
                        attribs.push(wgl_ext::CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB);
                    }
                }
            }

            if config.debug {
                flags = flags | wgl_ext::CONTEXT_DEBUG_BIT_ARB;
            }

            if extra.gl_robustness != OpenGLRobustness::DontCare {
                if self.has_wgl_extension("WGL_ARB_create_context_robustness") {
                    if extra.gl_robustness == OpenGLRobustness::NoReset {
                        attribs.push(wgl_ext::CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB);
                        attribs.push(wgl_ext::NO_RESET_NOTIFICATION_ARB);
                    } else if extra.gl_robustness == OpenGLRobustness::LoseContextOnReset {
                        attribs.push(wgl_ext::CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB);
                        attribs.push(wgl_ext::LOSE_CONTEXT_ON_RESET_ARB);
                    }
                    flags = flags | wgl_ext::CONTEXT_ROBUST_ACCESS_BIT_ARB;
                }
            }


            if extra.gl_release != OpenGLRelease::DontCare {
                if self.has_wgl_extension("WGL_ARB_context_flush_control") {
                    if extra.gl_release == OpenGLRelease::None {
                        attribs.push(wgl_ext::CONTEXT_RELEASE_BEHAVIOR_ARB);
                        attribs.push(wgl_ext::CONTEXT_RELEASE_BEHAVIOR_NONE_ARB);
                    } else if extra.gl_release == OpenGLRelease::Flush {
                        attribs.push(wgl_ext::CONTEXT_RELEASE_BEHAVIOR_ARB);
                        attribs.push(wgl_ext::CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB);
                    }
                }
            }

            /*if config.gl_noerror {
                if self.has_extension("WGL_ARB_create_context_no_error") {
                    attribs.push(WGL_CONTEXT_OPENGL_NO_ERROR_ARB);
                    attribs.push(1);
                }
            }*/

            if extra.gl_version != (0, 0) {
                attribs.push(wgl_ext::CONTEXT_MAJOR_VERSION_ARB);
                attribs.push(extra.gl_version.0 as u32);
                attribs.push(wgl_ext::CONTEXT_MINOR_VERSION_ARB);
                attribs.push(extra.gl_version.1 as u32);
            }

            if flags != 0 {
                attribs.push(wgl_ext::CONTEXT_FLAGS_ARB);
                attribs.push(flags);
            }

            // and of attribut list
            attribs.push(0);
            attribs.push(0);

            let hglrc = ffi!(self.wgl_ext.CreateContextAttribsARB(self.hdc as *const c_void,
                                                                  share as *const c_void,
                                                                  attribs.as_ptr() as *const i32));
            if hglrc.is_null() {
                return Err(Error::WindowCreationError(format!("WGL: Context creation failed: {}", io::Error::last_os_error())));
            }

            Ok(hglrc as winapi::HGLRC)
        } else {
            let hglrc = ffi!(wgl::CreateContext(self.hdc as *const c_void));
            if hglrc.is_null() {
                return Err(Error::WindowCreationError(format!("WGL: Context creation failed: {}", io::Error::last_os_error())));
            }

            if share.is_null() {
                if ffi!(wgl::ShareLists(share as *const c_void, self.hglrc as *const c_void)) == 0 {
                    return Err(Error::WindowCreationError(format!("WGL: Failed to enable sharing with specified OpenGL context")));
                }
            }
            Ok(hglrc as winapi::HGLRC)
        }
    }

    fn release_context(&mut self) {
        if !self.hglrc.is_null() {
            assert!(!self.hdc.is_null());
            ffi!(wgl::MakeCurrent(self.hdc as *const c_void, ptr::null_mut()));
            ffi!(wgl::DeleteContext(self.hglrc as *const c_void));
            self.hglrc = 0 as winapi::HGLRC;
        }

        if !self.hdc.is_null() {
            ffi!(user32::ReleaseDC(self.hwnd, self.hdc));
            self.hdc = 0 as winapi::HDC;
        }
    }

    fn load_gl_functions(&mut self) -> Result<(), Error> {
        gl::load_with(|symbol| {
            let addr = self.get_proc_address(symbol) as *const winapi::c_void;
            //println!("loading {:?}= {:p}", symbol, addr);
            addr
        });
        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        //println!("Context dropped");
        self.release_context();
        ffi!(kernel32::FreeLibrary(self.gl_library));
    }
}

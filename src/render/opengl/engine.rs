use std::mem;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use render::kernel32;
use render::user32;
use render::winapi;

use render::*;

pub struct GLEngineGlobals {
    is_initialized: bool,
    hinstance: winapi::HINSTANCE,
    window_class_name: Vec<u16>,
}

impl GLEngineGlobals {
    pub fn new() -> GLEngineGlobals {
        let window_class_name = OsStr::new("Dragorust")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let hinstance = unsafe { kernel32::GetModuleHandleW(ptr::null()) };

        GLEngineGlobals {
            is_initialized: false,
            hinstance: hinstance,
            window_class_name: window_class_name,
        }
    }

    fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn init(&mut self) -> Result<(), EngineError> {
        if self.is_initialized {
            return Ok(());
        }

        let class = winapi::WNDCLASSEXW {
            cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
            style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_OWNDC,
            lpfnWndProc: None,
            //Some(events_loop::callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: self.hinstance,
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: self.window_class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };

        let res = unsafe { user32::RegisterClassExW(&class) };
        if res == 0 {
            return Err(EngineError::InitializeError("".to_string()));
        }

        self.is_initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) {
        if !self.is_initialized {
            return;
        }

        unsafe { user32::UnregisterClassW(self.window_class_name.as_ptr(), self.hinstance); }
        self.is_initialized = false;
    }
}

static mut ENIGINE_GLOBALS: *mut GLEngineGlobals = 0 as *mut GLEngineGlobals; // ptr::null_mut();

pub struct GLEngine;

impl GLEngine {
    pub fn init() -> Result<(), EngineError> {
        unsafe {
            if ENIGINE_GLOBALS == ptr::null_mut() {
                ENIGINE_GLOBALS = Box::into_raw(Box::new(GLEngineGlobals::new()));
            }
            (*ENIGINE_GLOBALS).init()
        }
    }

    pub fn is_initialzed() -> bool {
        unsafe { ENIGINE_GLOBALS != ptr::null_mut() && (*ENIGINE_GLOBALS).is_initialized() }
    }

    pub fn shutdown() {
        unsafe {
            if ENIGINE_GLOBALS != ptr::null_mut() {
                (*ENIGINE_GLOBALS).shutdown();
                drop(Box::from_raw(ENIGINE_GLOBALS));
                ENIGINE_GLOBALS = ptr::null_mut();
            }
        }
    }

    pub fn get_window_class_name() -> Vec<u16> {
        assert!(Self::is_initialzed());
        unsafe { ENIGINE_GLOBALS.window_class_name }
    }

    pub fn get_instance() -> winapi::HINSTANCE {
        assert!(Self::is_initialzed());
        unsafe { ENIGINE_GLOBALS.hinstance }
    }
}

pub type EngineImpl = GLEngine;

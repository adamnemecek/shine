use std::mem;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::time::Duration;

use render::kernel32;
use render::user32;
use render::winapi;

use render::*;
use render::opengl::window::*;

pub unsafe extern "system" fn wnd_proc(hwnd: winapi::HWND, msg: winapi::UINT,
                                       wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                                       -> winapi::LRESULT
{
    let win_ptr = user32::GetWindowLongPtrW(hwnd, 0);
    if win_ptr == 0 {
        return user32::DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    {
        let mut win = GLWindow::new_from_raw(win_ptr);
        let win_hwnd = win.get_hwnd();
        if win_hwnd == hwnd {
            return win.handle_os_message(hwnd, msg, wparam, lparam)
        }
    }

    user32::DefWindowProcW(hwnd, msg, wparam, lparam)
}

pub struct GLEngine {
    hinstance: winapi::HINSTANCE,
    window_class_name: Vec<u16>,
}

impl GLEngine {
    pub fn new() -> Result<GLEngine, EngineError> {
        let window_class_name = OsStr::new("Dragorust")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let hinstance = unsafe { kernel32::GetModuleHandleW(ptr::null()) };

        let class = winapi::WNDCLASSEXW {
            cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
            style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_OWNDC,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: mem::size_of::<*mut GLWindow>() as i32,
            hInstance: hinstance,
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: window_class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };

        let res = unsafe { user32::RegisterClassExW(&class) };
        if res == 0 {
            return Err(EngineError::InitializeError("".to_string()));
        }

        Ok(GLEngine {
            hinstance: hinstance,
            window_class_name: window_class_name,
        })
    }

    pub fn quit(&mut self) {
        unsafe { user32::UnregisterClassW(self.window_class_name.as_ptr(), self.hinstance); }
    }

    pub fn dispatch_event(&mut self, timeout: Option<Duration>) -> bool {
        unsafe {
            let mut msg = mem::uninitialized();
            if user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                // Only happens if the message is `WM_QUIT`.
                //debug_assert_eq!(msg.message, winapi::WM_QUIT);
                return false;
            }

            /// messages are delegated to the window in the window proc
            user32::TranslateMessage(&msg);
            user32::DispatchMessageW(&msg);
        }
        true
    }

    pub fn get_window_class_name(&self) -> &Vec<u16> {
        &self.window_class_name
    }

    pub fn get_instance(&self) -> winapi::HINSTANCE {
        self.hinstance
    }
}

pub type EngineImpl = GLEngine;

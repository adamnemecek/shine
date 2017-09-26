use std::mem;
use std::ptr;
use std::cell::Cell;
//use std::i32;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use render::kernel32;
use render::user32;
use render::winapi;

use render::*;
use render::opengl::window::window::*;

pub unsafe extern "system" fn wnd_proc(hwnd: winapi::HWND, msg: winapi::UINT,
                                       wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                                       -> winapi::LRESULT
{
    let win_ptr = user32::GetWindowLongPtrW(hwnd, 0);
    if win_ptr == 0 {
        return user32::DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    return GLWindow::handle_os_message(win_ptr, hwnd, msg, wparam, lparam)
}


/// Engine implementation for Windows.
pub struct GLEngine {
    hinstance: winapi::HINSTANCE,
    window_class_name: Vec<u16>,

    // Number of active/non-closed windows
    window_count: Cell<i32>,

}

impl GLEngine {
    pub fn new() -> Result<GLEngine, Error> {
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
            return Err(Error::InitializeError(format!("")));
        }

        Ok(GLEngine {
            hinstance: hinstance,
            window_class_name: window_class_name,

            // While no window is created it is set an extremal value, not to terminate dispatch event before
            // the window creation has terminated.
            window_count: Cell::new(i32::max_value()),
        })
    }

    pub fn quit(&mut self) {
        unsafe { user32::PostQuitMessage(0); }
    }

    pub fn dispatch_event(&self, timeout: DispatchTimeout) -> bool {
        let mut new_window_count = self.window_count.get();

        unsafe {
            let mut msg: winapi::MSG = mem::zeroed();

            match timeout {
                DispatchTimeout::Immediate => {
                    if user32::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) == 0 {
                        return true;
                    }
                }

                DispatchTimeout::Infinite => {
                    if user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                        // Only happens if the message is `WM_QUIT`.
                        //debug_assert_eq!(msg.message, winapi::WM_QUIT);
                        return false;
                    }
                }

                DispatchTimeout::Time(timeout) => {
                    if user32::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) == 0 {
                        let secs_part = (timeout.as_secs() * 1000) as i64;
                        let nanos_part = (timeout.subsec_nanos() / 1000_000) as i64;
                        let timeout_ms = secs_part + nanos_part;

                        // no pending message, let's wait for some
                        if user32::MsgWaitForMultipleObjects(0, ptr::null_mut(), winapi::FALSE, timeout_ms as u32, winapi::QS_ALLEVENTS) != winapi::WAIT_OBJECT_0 {
                            return true;
                        }

                        if user32::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) == 0 {
                            // it shall never happen, but who knows, stay on the safe side :)
                            return true;
                        }
                    }
                }
            }

            if msg.message == WM_DR_WINDOW_CREATED {
                println!("WM_DR_WINDOW_CREATED");
                if new_window_count == i32::max_value() {
                    new_window_count = 1;
                } else {
                    new_window_count += 1;
                }
            } else if msg.message == WM_DR_WINDOW_DESTROYED {
                println!("WM_DR_WINDOW_DESTROYED");
                new_window_count -= 1;
            }

            // messages are delegated to the window in the window proc
            user32::TranslateMessage(&msg);
            user32::DispatchMessageW(&msg);
        }

        self.window_count.set(new_window_count);
        new_window_count > 0
    }


    pub fn get_window_class_name(&self) -> &Vec<u16> {
        &self.window_class_name
    }

    pub fn get_instance(&self) -> winapi::HINSTANCE {
        self.hinstance
    }
}

impl Drop for GLEngine {
    fn drop(&mut self) {
        unsafe { user32::UnregisterClassW(self.window_class_name.as_ptr(), self.hinstance); }
    }
}

pub type EngineImpl = GLEngine;

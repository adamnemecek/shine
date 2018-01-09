use std::ptr;
use std::io;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use user32;
use winapi;
use core::*;

/// A dummy window created to detect gl capabilities.
/// It has no message handling and usually destroyed immediately after creation.
pub struct DummyWindow {
    pub hwnd: winapi::HWND,
    pub hdc: winapi::HDC,
}

impl DummyWindow {
    pub fn new(app_instance: winapi::HINSTANCE, hwnd: winapi::HWND) -> Result<DummyWindow, Error> {
        unsafe {
            // some default window style
            let (ex_style, style) = (winapi::WS_EX_APPWINDOW,
                                     winapi::WS_POPUP | winapi::WS_CLIPSIBLINGS | winapi::WS_CLIPCHILDREN);

            // getting placement of the real window
            let rect = {
                let mut placement: winapi::WINDOWPLACEMENT = mem::zeroed();
                placement.length = mem::size_of::<winapi::WINDOWPLACEMENT>() as winapi::UINT;
                if user32::GetWindowPlacement(hwnd, &mut placement) == 0 {
                    return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetWindowPlacement: {}", io::Error::last_os_error())));
                }
                placement.rcNormalPosition
            };

            // getting the class info for the real window
            let class_name = {
                let mut class: winapi::WNDCLASSEXW = mem::zeroed();
                let mut class_name = [0u16; 128];
                if user32::GetClassNameW(hwnd, class_name.as_mut_ptr(), 128) == 0 {
                    return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetClassNameW: {}", io::Error::last_os_error())));
                }
                if user32::GetClassInfoExW(app_instance, class_name.as_ptr(), &mut class) == 0 {
                    return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetClassInfoExW: {}", io::Error::last_os_error())));
                }

                let class_name = OsStr::new("WGLDummy").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
                class.cbSize = mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT;
                class.lpszClassName = class_name.as_ptr();
                class.lpfnWndProc = Some(user32::DefWindowProcW);

                user32::RegisterClassExW(&class);
                class_name
            };

            // this dummy window should match the real one enough to get the same OpenGL driver
            let title = OsStr::new("dummy window").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
            let win = user32::CreateWindowExW(ex_style, class_name.as_ptr(),
                                              title.as_ptr() as winapi::LPCWSTR, style,
                                              winapi::CW_USEDEFAULT, winapi::CW_USEDEFAULT,
                                              rect.right - rect.left,
                                              rect.bottom - rect.top,
                                              ptr::null_mut(), ptr::null_mut(),
                                              app_instance,
                                              ptr::null_mut());
            if win.is_null() {
                return Err(Error::WindowCreationError(format!("Dummy window creation failed; CreateWindowEx: {}", io::Error::last_os_error())));
            }

            let hdc = user32::GetDC(win);
            if hdc.is_null() {
                return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetDC: {}", io::Error::last_os_error())));
            }

            Ok(DummyWindow {
                hwnd: win,
                hdc: hdc
            })
        }
    }
}

impl Drop for DummyWindow {
    fn drop(&mut self) {
        //println!("DummyWindow dropped");
        unsafe {
            user32::ReleaseDC(self.hwnd, self.hdc);
            user32::DestroyWindow(self.hwnd);
        }
    }
}

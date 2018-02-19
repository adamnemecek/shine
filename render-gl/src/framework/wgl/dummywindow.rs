use std::ptr;
use std::io;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::shared::ntdef::*;
use core::*;

/// A dummy window created to detect gl capabilities.
/// It has no message handling and usually destroyed immediately after creation.
pub struct DummyWindow {
    pub hwnd: HWND,
    pub hdc: HDC,
}

impl DummyWindow {
    pub fn new(app_instance: HINSTANCE, hwnd: HWND) -> Result<DummyWindow, Error> {
        // some default window style
        let (ex_style, style) = (WS_EX_APPWINDOW,
                                 WS_POPUP | WS_CLIPSIBLINGS | WS_CLIPCHILDREN);

        // getting placement of the real window
        let rect = {
            let mut placement: WINDOWPLACEMENT = unsafe { mem::zeroed() };
            placement.length = mem::size_of::<WINDOWPLACEMENT>() as UINT;
            if ffi!(GetWindowPlacement(hwnd, &mut placement)) == 0 {
                return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetWindowPlacement: {}", io::Error::last_os_error())));
            }
            placement.rcNormalPosition
        };

        // getting the class info for the real window
        let class_name = {
            let mut class: WNDCLASSEXW = unsafe { mem::zeroed() };
            let mut class_name = [0u16; 128];
            if ffi!(GetClassNameW(hwnd, class_name.as_mut_ptr(), 128)) == 0 {
                return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetClassNameW: {}", io::Error::last_os_error())));
            }
            if ffi!(GetClassInfoExW(app_instance, class_name.as_ptr(), &mut class)) == 0 {
                return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetClassInfoExW: {}", io::Error::last_os_error())));
            }

            let class_name = OsStr::new("WGLDummy").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
            class.cbSize = mem::size_of::<WNDCLASSEXW>() as UINT;
            class.lpszClassName = class_name.as_ptr();
            class.lpfnWndProc = Some(DefWindowProcW);

            ffi!(RegisterClassExW(&class));
            class_name
        };

        // this dummy window should match the real one enough to get the same OpenGL driver
        let title = OsStr::new("dummy window").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
        let win = ffi!(CreateWindowExW(ex_style, class_name.as_ptr(),
                                               title.as_ptr() as LPCWSTR, style,
                                               CW_USEDEFAULT, CW_USEDEFAULT,
                                               rect.right - rect.left,
                                               rect.bottom - rect.top,
                                               ptr::null_mut(), ptr::null_mut(),
                                               app_instance,
                                               ptr::null_mut()));
        if win.is_null() {
            return Err(Error::WindowCreationError(format!("Dummy window creation failed; CreateWindowEx: {}", io::Error::last_os_error())));
        }

        let hdc = ffi!(GetDC(win));
        if hdc.is_null() {
            return Err(Error::WindowCreationError(format!("Dummy window creation failed; GetDC: {}", io::Error::last_os_error())));
        }

        Ok(DummyWindow {
            hwnd: win,
            hdc: hdc,
        })
    }
}

impl Drop for DummyWindow {
    fn drop(&mut self) {
        //println!("DummyWindow dropped");
        ffi!(ReleaseDC(self.hwnd, self.hdc));
        ffi!(DestroyWindow(self.hwnd));
    }
}

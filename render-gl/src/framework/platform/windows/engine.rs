use std::mem;
use std::ptr;
use std::sync;
use std::sync::mpsc;
use std::cell::Cell;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::um::libloaderapi::*;
use winapi::um::winbase::*;
use core::*;
use framework::*;

mod vkey;

use self::vkey::*;


/// Window message handler callback function
pub extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
    let send_queue = ffi!(GetWindowLongPtrW(hwnd, 0));
    if send_queue == 0 {
        return ffi!(DefWindowProcW(hwnd, msg, wparam, lparam));
    }

    let send_queue = unsafe { Box::from_raw(send_queue as *mut mpsc::Sender<WindowCmd>) };

    let mut result: Option<LRESULT> = None;
    match msg {
        win_messages::WM_DR_WINDOW_CREATED => {
            let barrier = sync::Arc::new(sync::Barrier::new(2));
            send_queue.send(WindowCmd::Sync(WindowCommand::SurfaceReady, barrier.clone())).unwrap();
            barrier.wait();
        }

        WM_CLOSE => {
            let barrier = sync::Arc::new(sync::Barrier::new(2));
            send_queue.send(WindowCmd::Sync(WindowCommand::SurfaceLost, barrier.clone())).unwrap();
            barrier.wait();
        }

        WM_DESTROY => {
            ffi!(PostMessageW(ptr::null_mut(), win_messages::WM_DR_WINDOW_DESTROYED, 0, 0));
        }

        WM_SIZE => {
            let w = LOWORD(lparam as DWORD) as i32;
            let h = HIWORD(lparam as DWORD) as i32;

            let size = Size { width: w, height: h };

            //let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            //ffi!(GetWindowRect(win.hwnd, &mut rect));
            //let size = Size {
            //    width: rect.right - rect.left,
            //    height: rect.bottom - rect.top,
            //};

            send_queue.send(WindowCmd::Async(WindowCommand::Resize(size))).unwrap();
            result = Some(0);
        }

        WM_MOVE => {
            let x = LOWORD(lparam as DWORD) as i32;
            let y = HIWORD(lparam as DWORD) as i32;
            let position = Position { x: x, y: y };
            send_queue.send(WindowCmd::Async(WindowCommand::Move(position))).unwrap();
            result = Some(0);
        }

        WM_KEYDOWN | WM_SYSKEYDOWN => {
            if msg == WM_SYSKEYDOWN && wparam as i32 == VK_F4 {
                // pass close by F4 key to windows
                result = None;
            } else {
                let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                send_queue.send(WindowCmd::Async(WindowCommand::KeyboardDown(sc, vkey))).unwrap();
                result = Some(0);
            }
        }

        WM_KEYUP | WM_SYSKEYUP => {
            if msg == WM_SYSKEYUP && wparam as i32 == VK_F4 {
                // pass close by F4 key
                result = None;
            } else {
                let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                send_queue.send(WindowCmd::Async(WindowCommand::KeyboardUp(sc, vkey))).unwrap();
                result = Some(0);
            }
        }

        _ => {}
    }

    mem::forget(send_queue);
    if let Some(res) = result {
        return res;
    }
    ffi!(DefWindowProcW(hwnd, msg, wparam, lparam))
}


/// Engine implementation for Windows.
pub struct GLEngine {
    hinstance: HINSTANCE,
    window_class_name: Vec<u16>,

    // Number of active/non-closed windows
    window_count: Cell<i32>,

}

impl GLEngine {
    pub fn new() -> Result<Box<GLEngine>, Error> {
        let window_class_name = OsStr::new("shine")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let hinstance = ffi!(GetModuleHandleW(ptr::null()));

        let class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
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

        let res = ffi!(RegisterClassExW(&class));
        if res == 0 {
            return Err(Error::InitializeError(format!("")));
        }

        Ok(Box::new(GLEngine {
            hinstance: hinstance,
            window_class_name: window_class_name,

            // While no window is created it is set an extremal value, not to terminate dispatch event before
            // the window creation has terminated.
            window_count: Cell::new(i32::max_value()),
        }))
    }

    pub fn quit(&self) {
        ffi!(PostQuitMessage(0));
    }

    pub fn dispatch_event(&self, timeout: DispatchTimeout) -> bool {
        let mut new_window_count = self.window_count.get();

        let mut msg: MSG = unsafe { mem::zeroed() };

        match timeout {
            DispatchTimeout::Immediate => {
                if ffi!(PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE)) == 0 {
                    return true;
                }
            }

            DispatchTimeout::Infinite => {
                if ffi!(GetMessageW(&mut msg, ptr::null_mut(), 0, 0)) == 0 {
                    // Only happens if the message is `WM_QUIT`.
                    //debug_assert_eq!(msg.message, WM_QUIT);
                    return false;
                }
            }

            DispatchTimeout::Time(timeout) => {
                if ffi!(PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE)) == 0 {
                    let secs_part = (timeout.as_secs() * 1000) as i64;
                    let nanos_part = (timeout.subsec_nanos() / 1000_000) as i64;
                    let timeout_ms = secs_part + nanos_part;

                    // no pending message, let's wait for some
                    if ffi!(MsgWaitForMultipleObjects(0, ptr::null_mut(), FALSE, timeout_ms as u32, QS_ALLEVENTS)) != WAIT_OBJECT_0 {
                        return true;
                    }

                    if ffi!(PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE)) == 0 {
                        // it shall never happen, but who knows, stay on the safe side :)
                        return true;
                    }
                }
            }
        }

        if msg.message == win_messages::WM_DR_WINDOW_CREATED {
            //println!("dispatching WM_DR_WINDOW_CREATED, new_window_count: {}", new_window_count);
            if new_window_count == i32::max_value() {
                new_window_count = 1;
            } else {
                new_window_count += 1;
            }
        } else if msg.message == win_messages::WM_DR_WINDOW_DESTROYED {
            new_window_count -= 1;
            //println!("dispatching  WM_DR_WINDOW_DESTROYED, new_window_count: {}", new_window_count);
        }

        // messages are delegated to the window in the window proc
        ffi!(TranslateMessage(&msg));
        ffi!(DispatchMessageW(&msg));


        self.window_count.set(new_window_count);
        new_window_count > 0
    }


    pub fn get_window_class_name(&self) -> &Vec<u16> {
        &self.window_class_name
    }

    pub fn get_instance(&self) -> HINSTANCE {
        self.hinstance
    }
}

impl Drop for GLEngine {
    fn drop(&mut self) {
        ffi!(UnregisterClassW(self.window_class_name.as_ptr(), self.hinstance));
    }
}


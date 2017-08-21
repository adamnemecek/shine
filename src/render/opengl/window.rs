use std::ptr;
use std::io;
use std::mem;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use render::user32;
use render::winapi;

use render::*;
use render::opengl::engine::*;


/// User window message indicating the window creating has completed and surface ready
/// callback can be called.
pub const WM_DR_READY: winapi::UINT = winapi::WM_USER + 1;


/// Returns the window style for the specified settings
fn get_window_style(settings: &WindowSettings) -> u32 {
    let mut style = winapi::WS_CLIPSIBLINGS | winapi::WS_CLIPCHILDREN;

    //if settings.monitor {
    //   style |= WS_POPUP;
    //} else {
    style |= winapi::WS_SYSMENU | winapi::WS_MINIMIZEBOX;

    if settings.decorated {
        style |= winapi::WS_CAPTION;
    }

    if settings.resizable {
        style |= winapi::WS_MAXIMIZEBOX | winapi::WS_THICKFRAME;
    } else {
        style |= winapi::WS_POPUP;
    }

    /*if settings.maximized {
        style |= winapi::WS_MAXIMIZE;
    }*/

    style
}


/// Returns the extended window style for the specified settings
#[allow(unused_variables)]
fn get_window_exstyle(settings: &WindowSettings) -> u32 {
    let style = winapi::WS_EX_APPWINDOW;

    //if settings.monitor  {
    //    style |= winapi::WS_EX_TOPMOST;
    //}

    style
}

fn get_full_window_size(style: u32, exstyle: u32, client_size: Size) -> Size {
    use render::winapi::*;
    use render::user32::*;

    let mut rect = RECT {
        top: 0,
        left: 0,
        bottom: client_size.height as winapi::LONG,
        right: client_size.width as winapi::LONG
    };

    //todo: error handling
    unsafe { AdjustWindowRectEx(&mut rect, style, winapi::FALSE, exstyle); }

    Size {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32
    }
}

pub struct GLWindowWrapper {
    hwnd: winapi::HWND,
    size: Size,
    surface_handler: Option<Rc<RefCell<SurfaceEventHandler>>>,
}

impl Drop for GLWindowWrapper {
    fn drop(&mut self) {
        println!("GLWindowWrapper dropped")
    }
}

pub struct GLWindow(Rc<RefCell<GLWindowWrapper>>);

impl GLWindow {
    pub fn new(settings: WindowSettings, engine: &mut Engine) -> Result<GLWindow, CreationError> {
        // create window
        let ref mut engine = engine.platform;
        let style = get_window_style(&settings);
        let exstyle = get_window_exstyle(&settings);
        let xpos = winapi::CW_USEDEFAULT;
        let ypos = winapi::CW_USEDEFAULT;
        let full_size = get_full_window_size(style, exstyle, settings.size);
        let title = OsStr::new(&settings.title)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();
        let hwnd = unsafe {
            user32::CreateWindowExW(exstyle,
                                    engine.get_window_class_name().as_ptr(),
                                    title.as_ptr() as winapi::LPCWSTR,
                                    style,
                                    xpos, ypos,
                                    full_size.width as winapi::LONG, full_size.height as winapi::LONG,
                                    ptr::null_mut(), // No parent window
                                    ptr::null_mut(), // No window menu
                                    engine.get_instance(),
                                    ptr::null_mut())
        };
        println!("created hwnd {:?}", hwnd);

        if hwnd.is_null() {
            return Err(CreationError::OsError(format!("CreateWindowEx function failed: {}", io::Error::last_os_error())));
        }

        let win =
            Rc::new(RefCell::new(GLWindowWrapper {
                hwnd: hwnd,
                size: settings.size,
                surface_handler: None,
            }));

        //connect the OS and rust window
        unsafe {
            // the connection string is same as the window class name for simplicity
            let win_ptr = Rc::into_raw(win.clone());
            user32::SetWindowLongPtrW(hwnd, 0, win_ptr as i64);
        }

        //if _glfw_ChangeWindowMessageFilterEx {
        //_glfw_ChangeWindowMessageFilterEx(window->win32.handle, WM_DROPFILES, MSGFLT_ALLOW, NULL);
        //_glfw_ChangeWindowMessageFilterEx(window->win32.handle, WM_COPYDATA, MSGFLT_ALLOW, NULL);
        //_glfw_ChangeWindowMessageFilterEx(window->win32.handle, WM_COPYGLOBALDATA, MSGFLT_ALLOW, NULL);
        //}
        //DragAcceptFiles(window->win32.handle, TRUE);
        // create context

        unsafe {
            user32::ShowWindow(hwnd, winapi::SW_SHOW);

            // The native window creation completes before our callback is injected into the system,
            // thus  a delayed message is sent, that is delivered only when the message loop
            // has started. (engine.dispatch_events)
            user32::PostMessageW(hwnd, WM_DR_READY, 0, 0);
        }

        Ok(GLWindow(win))
    }

    pub unsafe fn new_from_raw(ptr: winapi::LONG_PTR) -> GLWindow {
        assert!(ptr != 0);
        let rc = Rc::from_raw(ptr as *mut RefCell<GLWindowWrapper>);

        // rc created by Rc::from_raw will decrement ref count on drop
        // but we don't want to lose the window yet, so "leak" memory here.
        // We will get it back during the close/destroy process.
        Rc::into_raw(rc.clone());

        GLWindow(rc)
    }

    fn to_window(&self) -> Window {
        Window { platform: GLWindow(self.0.clone()) }
    }

    pub fn set_surface_handler<H: SurfaceEventHandler>(&mut self, handler: H) {
        let ref mut window = self.0.borrow_mut();
        window.surface_handler = Some(Rc::new(RefCell::new(handler)));
    }

    pub fn close(&mut self) {}

    pub fn is_closed(&self) -> bool {
        false
    }

    pub fn size(&self) -> Size {
        let ref window = self.0.borrow();
        window.size
    }

    pub fn draw_size(&self) -> Size {
        Size { width: 0, height: 0 }
    }

    pub fn start_render(&self) -> Result<(), ContextError> {
        Ok(())
    }

    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        Ok(())
    }

    pub fn end_render(&self) -> Result<(), ContextError> {
        Ok(())
    }

    pub fn get_hwnd(&self) -> winapi::HWND {
        let ref window = self.0.borrow();
        window.hwnd
    }

    pub fn handle_os_message(&mut self, hwnd: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                             -> winapi::LRESULT {
        {
            let ref window = self.0.borrow();
            assert!(window.hwnd == hwnd);
        }

        let mut result: Option<winapi::LRESULT> = None;
        {
            match msg {
                WM_DR_READY => {
                    let handler;
                    {
                        let ref window = self.0.borrow();
                        handler = window.surface_handler.clone();
                    }
                    if let Some(ref handler) = handler {
                        handler.borrow_mut().on_ready(&mut self.to_window());
                    }
                }

                winapi::WM_CLOSE => {
                    let handler;
                    {
                        let ref window = self.0.borrow();
                        handler = window.surface_handler.clone();
                    }
                    if let Some(ref handler) = handler {
                        handler.borrow_mut().on_lost(&mut self.to_window());
                    }
                }

                winapi::WM_DESTROY => {}
                _ => {}
            }
        }

        if let Some(res) = result {
            return res;
        }
        unsafe { user32::DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}

pub type WindowImpl = GLWindow;

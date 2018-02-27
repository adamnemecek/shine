use std::ptr;
use std::io;
use std::mem;
use std::sync::mpsc;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::ntdef::{LONG, LPCWSTR};
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use core::*;
use framework::*;
use resources::*;

pub mod win_messages {
    use super::*;

    /// User window message indicating the window creating has completed and surface ready
    /// callback can be called.
    pub const WM_DR_WINDOW_CREATED: UINT = WM_USER + 1;
    pub const WM_DR_WINDOW_DESTROYED: UINT = WM_USER + 2;
}


/// Returns the window style for the specified settings
fn get_window_style(settings: &PlatformWindowSettings) -> u32 {
    let mut style = WS_CLIPSIBLINGS | WS_CLIPCHILDREN;

    //if settings.monitor {
    //   style |= WS_POPUP;
    //} else {
    style |= WS_SYSMENU | WS_MINIMIZEBOX;

    if settings.decorated {
        style |= WS_CAPTION;
    }

    if settings.resizable {
        style |= WS_MAXIMIZEBOX | WS_THICKFRAME;
    } else {
        style |= WS_POPUP;
    }

    /*if settings.maximized {
        style |= WS_MAXIMIZE;
    }*/

    style
}


/// Returns the extended window style for the specified settings
#[allow(unused_variables)]
fn get_window_exstyle(settings: &PlatformWindowSettings) -> u32 {
    let style = WS_EX_APPWINDOW;

    //if settings.monitor  {
    //    style |= WS_EX_TOPMOST;
    //}

    style
}

fn get_full_window_size(style: u32, exstyle: u32, client_size: Size) -> Size {
    let mut rect = RECT {
        top: 0,
        left: 0,
        bottom: client_size.height as LONG,
        right: client_size.width as LONG,
    };

    //todo: error handling
    ffi!(AdjustWindowRectEx(&mut rect, style, FALSE, exstyle));

    Size {
        width: rect.right - rect.left,
        height: rect.bottom - rect.top,
    }
}


/// Window status from opening to closed.
#[derive(PartialEq)]
enum WindowState {
    WaitingOpen,
    Open,
    WaitingClose,
    Closed,
}


/// Structure to store platform dependent data associated to a Window.
pub struct GLWindow {
    hwnd: HWND,
    state: WindowState,
    size: Size,
    position: Position,
    context: GLContext,
    backend: GLBackend,
}

unsafe impl Send for GLWindow {}

impl GLWindow {
    pub fn new(settings: &PlatformWindowSettings, engine: &GLEngine, msg: mpsc::Sender<WindowCmd>) -> Result<GLWindow, Error> {
        // create OS window
        let app_instance = engine.get_instance();

        let (style, exstyle) = (get_window_style(settings), get_window_exstyle(settings));
        let (xpos, ypos) = (CW_USEDEFAULT, CW_USEDEFAULT);
        let full_size = get_full_window_size(style, exstyle, settings.size);
        let title = OsStr::new(&settings.title).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
        let hwnd = ffi!(CreateWindowExW(exstyle,
                                        engine.get_window_class_name().as_ptr(),
                                        title.as_ptr() as LPCWSTR,
                                        style,
                                        xpos, ypos,
                                        full_size.width as LONG, full_size.height as LONG,
                                        ptr::null_mut(), // No parent window
                                        ptr::null_mut(), // No window menu
                                        app_instance,
                                        ptr::null_mut()));
        //println!("Os window created, hwnd: {:?}", hwnd);
        if hwnd.is_null() {
            return Err(Error::WindowCreationError(format!("Window: CreateWindowEx function failed: {}", io::Error::last_os_error())));
        }

        //create context
        let context = match GLContext::new(app_instance, hwnd, settings) {
            Err(err) => {
                ffi!(DestroyWindow(hwnd));
                return Err(err);
            }

            Ok(context) => { context }
        };

        let data = GLWindow {
            hwnd: hwnd,
            state: WindowState::WaitingOpen,
            size: settings.size,
            position: Position { x: 0, y: 0 },
            context: context,
            backend: GLBackend::new(),
        };

        //connect the OS and rust window
        {
            let msg = Box::new(msg);
            ffi!(SetWindowLongPtrW(hwnd, 0, mem::transmute(Box::into_raw(msg))));
        }

        // ready to show the window
        ffi!(ShowWindow(hwnd, SW_SHOW));

        // The native window creation completes before our callback is injected into the system,
        // thus  a delayed message is sent, that is delivered only when the message loop
        // has started. (engine.dispatch_events)
        ffi!(PostMessageW(hwnd, win_messages::WM_DR_WINDOW_CREATED, 0, 0));

        Ok(data)
    }

    pub fn is_closed(&self) -> bool {
        self.state == WindowState::Closed
            || self.hwnd == ptr::null_mut()
    }

    pub fn is_ready_to_render(&self) -> bool {
        let screen_size = self.backend.get_screen_size();
        self.state == WindowState::Open
            && (screen_size.width > 0 && screen_size.height > 0)
    }

    pub fn swap_buffers(&mut self) -> Result<(), Error> {
        self.backend.flush();
        try!(self.context.swap_buffers());
        Ok(())
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn pre_hook(&mut self, cmd: &WindowCommand) {
        //println!("hwnd:{:?} cmd:{:?}", self.hwnd, cmd);
        match cmd {
            &WindowCommand::SurfaceReady => {
                self.state = WindowState::Open;
                self.context.activate().unwrap();
            }

            &WindowCommand::Resize(ref window_size, ref client_size) => {
                self.size = window_size.clone();
                self.backend.set_screen_size(client_size.clone());
            }

            &WindowCommand::Move(ref position) => {
                self.position = position.clone();
            }

            _ => {}
        }
    }

    pub fn post_hook(&mut self, cmd: &WindowCommand) {
        match cmd {
            &WindowCommand::SurfaceReady => {
                self.backend.flush();
            }

            &WindowCommand::SurfaceLost => {
                self.backend.flush();
                self.state = WindowState::WaitingClose;
                self.context.deactivate().unwrap();
            }

            &WindowCommand::Closed => {
                self.state = WindowState::Closed;
                self.hwnd = ptr::null_mut();
            }

            _ => {}
        }
    }
}

impl Window<PlatformEngine> for GLWindow {
    fn close(&mut self) {
        ffi!(PostMessageW(self.hwnd, WM_CLOSE, 0, 0));
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_draw_size(&self) -> Size {
        self.backend.get_screen_size()
    }

    fn backend(&mut self) -> &mut GLBackend {
        &mut self.backend
    }
}

impl Drop for GLWindow {
    fn drop(&mut self) {
        //println!("GLWindow dropped");
        assert!(self.hwnd.is_null(), "Window is leaking, wait close before dropping it");
    }
}

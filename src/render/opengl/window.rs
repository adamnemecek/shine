use std::ptr;
use std::io;
use std::time::Duration;

use render::user32;
use render::winapi;

use render::*;
use render::opengl::engine::*;

pub struct GLWindow {
    hwnd: winapi::HWND,
}

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
fn get_window_exstyle(settings: &WindowSettings) -> u32 {
    use render::winapi::*;

    let style = WS_EX_APPWINDOW;

    //if (settings.monitor || settings.floating)
    // style |= WS_EX_TOPMOST;

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


impl GLWindow {
    pub fn new(settings: WindowSettings) -> Result<GLWindow, CreationError> {
        if !Engine::is_initialzed() {
            return Err(CreationError::EngineNotInitialized)
        }

        let style = get_window_style(&settings);
        let exstyle = get_window_exstyle(&settings);

        let xpos = winapi::CW_USEDEFAULT;
        let ypos = winapi::CW_USEDEFAULT;
        let full_size = get_full_window_size(style, exstyle, settings.size);


        let hwnd = unsafe {
            user32::CreateWindowExW(exstyle,
                                    GLEngine::get_window_class_name().as_ptr(),
                                    settings.title.as_ptr() as winapi::LPCWSTR,
                                    style,
                                    xpos, ypos,
                                    full_size.width as winapi::LONG, full_size.height as winapi::LONG,
                                    ptr::null_mut(), // No parent window
                                    ptr::null_mut(), // No window menu
                                    GLEngine::get_instance(),
                                    ptr::null_mut())
        };

        if hwnd.is_null() {
            return Err(CreationError::OsError(format!("CreateWindowEx function failed: {}", io::Error::last_os_error())));
        }

        Ok(GLWindow { hwnd: hwnd })
    }

    pub fn close(&mut self) {}

    pub fn is_closed(&self) -> bool {
        true
    }

    pub fn size(&self) -> Size {
        Size { width: 0, height: 0 }
    }

    pub fn draw_size(&self) -> Size {
        Size { width: 0, height: 0 }
    }

    pub fn wait_event(&mut self) -> Event {
        Event::Closed
    }

    pub fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Event> {
        Some(Event::Closed)
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
}

pub type WindowImpl = GLWindow;

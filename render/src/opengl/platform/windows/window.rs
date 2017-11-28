use std::ptr;
use std::io;
use std::mem;
use std::ffi::OsStr;
use std::rc::Rc;
use std::cell::RefCell;
use std::os::windows::ffi::OsStrExt;

use user32;
use winapi;
use backend::*;

use backend::opengl::context::Context;


/// User window message indicating the window creating has completed and surface ready
/// callback can be called.
pub const WM_DR_WINDOW_CREATED: winapi::UINT = winapi::WM_USER + 1;
pub const WM_DR_WINDOW_DESTROYED: winapi::UINT = winapi::WM_USER + 2;


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
    use winapi::*;
    use user32::*;

    let mut rect = RECT {
        top: 0,
        left: 0,
        bottom: client_size.height as winapi::LONG,
        right: client_size.width as winapi::LONG
    };

    //todo: error handling
    unsafe { AdjustWindowRectEx(&mut rect, style, winapi::FALSE, exstyle); }

    Size {
        width: rect.right - rect.left,
        height: rect.bottom - rect.top
    }
}


/// Maps from windows key code to our virtual key codes
fn vkeycode_to_element(wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> (ScanCode, Option<VirtualKeyCode>) {
    const MAPVK_VSC_TO_VK_EX: u32 = 3;

    let scancode = ((lparam >> 16) & 0xff) as u32;
    let extended = (lparam & 0x01000000) != 0;
    let vk = match wparam as i32 {
        winapi::VK_SHIFT => { unsafe { user32::MapVirtualKeyA(scancode, MAPVK_VSC_TO_VK_EX) as i32 } }
        winapi::VK_CONTROL => { if extended { winapi::VK_RCONTROL } else { winapi::VK_LCONTROL } }
        winapi::VK_MENU => { if extended { winapi::VK_RMENU } else { winapi::VK_LMENU } }
        other => other
    };

    // VK_* codes are documented here https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    (scancode, match vk {
        //winapi::VK_LBUTTON => Some(VirtualKeyCode::Lbutton),
        //winapi::VK_RBUTTON => Some(VirtualKeyCode::Rbutton),
        //winapi::VK_CANCEL => Some(VirtualKeyCode::Cancel),
        //winapi::VK_MBUTTON => Some(VirtualKeyCode::Mbutton),
        //winapi::VK_XBUTTON1 => Some(VirtualKeyCode::Xbutton1),
        //winapi::VK_XBUTTON2 => Some(VirtualKeyCode::Xbutton2),
        winapi::VK_BACK => Some(VirtualKeyCode::Backspace),
        winapi::VK_TAB => Some(VirtualKeyCode::Tab),
        //winapi::VK_CLEAR => Some(VirtualKeyCode::Clear),
        winapi::VK_RETURN => if extended { Some(VirtualKeyCode::NumpadEnter) } else { Some(VirtualKeyCode::Enter) },
        winapi::VK_LSHIFT => Some(VirtualKeyCode::LShift),
        winapi::VK_RSHIFT => Some(VirtualKeyCode::RShift),
        winapi::VK_LCONTROL => Some(VirtualKeyCode::LControl),
        winapi::VK_RCONTROL => Some(VirtualKeyCode::RControl),
        winapi::VK_LMENU => Some(VirtualKeyCode::LMenu),
        winapi::VK_RMENU => Some(VirtualKeyCode::RMenu),
        winapi::VK_PAUSE => Some(VirtualKeyCode::Pause),
        winapi::VK_CAPITAL => Some(VirtualKeyCode::CapsLock),
        //winapi::VK_KANA => Some(VirtualKeyCode::Kana),
        //winapi::VK_HANGUEL => Some(VirtualKeyCode::Hanguel),
        //winapi::VK_HANGUL => Some(VirtualKeyCode::Hangul),
        //winapi::VK_JUNJA => Some(VirtualKeyCode::Junja),
        //winapi::VK_FINAL => Some(VirtualKeyCode::Final),
        //winapi::VK_HANJA => Some(VirtualKeyCode::Hanja),
        //winapi::VK_KANJI => Some(VirtualKeyCode::Kanji),
        winapi::VK_ESCAPE => Some(VirtualKeyCode::Escape),
        //winapi::VK_CONVERT => Some(VirtualKeyCode::Convert),
        //winapi::VK_NONCONVERT => Some(VirtualKeyCode::NoConvert),
        //winapi::VK_ACCEPT => Some(VirtualKeyCode::Accept),
        //winapi::VK_MODECHANGE => Some(VirtualKeyCode::Modechange),
        winapi::VK_SPACE => Some(VirtualKeyCode::Space),
        winapi::VK_PRIOR => Some(VirtualKeyCode::PageUp),
        winapi::VK_NEXT => Some(VirtualKeyCode::PageDown),
        winapi::VK_END => Some(VirtualKeyCode::End),
        winapi::VK_HOME => Some(VirtualKeyCode::Home),
        winapi::VK_LEFT => Some(VirtualKeyCode::Left),
        winapi::VK_UP => Some(VirtualKeyCode::Up),
        winapi::VK_RIGHT => Some(VirtualKeyCode::Right),
        winapi::VK_DOWN => Some(VirtualKeyCode::Down),
        //winapi::VK_SELECT => Some(VirtualKeyCode::Select),
        //winapi::VK_PRINT => Some(VirtualKeyCode::Print),
        //winapi::VK_EXECUTE => Some(VirtualKeyCode::Execute),
        winapi::VK_SNAPSHOT => Some(VirtualKeyCode::PrintScreen),
        winapi::VK_INSERT => Some(VirtualKeyCode::Insert),
        winapi::VK_DELETE => Some(VirtualKeyCode::Delete),
        //winapi::VK_HELP => Some(VirtualKeyCode::Help),
        0x30 => Some(VirtualKeyCode::Key0),
        0x31 => Some(VirtualKeyCode::Key1),
        0x32 => Some(VirtualKeyCode::Key2),
        0x33 => Some(VirtualKeyCode::Key3),
        0x34 => Some(VirtualKeyCode::Key4),
        0x35 => Some(VirtualKeyCode::Key5),
        0x36 => Some(VirtualKeyCode::Key6),
        0x37 => Some(VirtualKeyCode::Key7),
        0x38 => Some(VirtualKeyCode::Key8),
        0x39 => Some(VirtualKeyCode::Key9),
        0x41 => Some(VirtualKeyCode::A),
        0x42 => Some(VirtualKeyCode::B),
        0x43 => Some(VirtualKeyCode::C),
        0x44 => Some(VirtualKeyCode::D),
        0x45 => Some(VirtualKeyCode::E),
        0x46 => Some(VirtualKeyCode::F),
        0x47 => Some(VirtualKeyCode::G),
        0x48 => Some(VirtualKeyCode::H),
        0x49 => Some(VirtualKeyCode::I),
        0x4A => Some(VirtualKeyCode::J),
        0x4B => Some(VirtualKeyCode::K),
        0x4C => Some(VirtualKeyCode::L),
        0x4D => Some(VirtualKeyCode::M),
        0x4E => Some(VirtualKeyCode::N),
        0x4F => Some(VirtualKeyCode::O),
        0x50 => Some(VirtualKeyCode::P),
        0x51 => Some(VirtualKeyCode::Q),
        0x52 => Some(VirtualKeyCode::R),
        0x53 => Some(VirtualKeyCode::S),
        0x54 => Some(VirtualKeyCode::T),
        0x55 => Some(VirtualKeyCode::U),
        0x56 => Some(VirtualKeyCode::V),
        0x57 => Some(VirtualKeyCode::W),
        0x58 => Some(VirtualKeyCode::X),
        0x59 => Some(VirtualKeyCode::Y),
        0x5A => Some(VirtualKeyCode::Z),
        //winapi::VK_LWIN => Some(VirtualKeyCode::Lwin),
        //winapi::VK_RWIN => Some(VirtualKeyCode::Rwin),
        winapi::VK_APPS => Some(VirtualKeyCode::Apps),
        winapi::VK_SLEEP => Some(VirtualKeyCode::Sleep),
        winapi::VK_NUMPAD0 => Some(VirtualKeyCode::Numpad0),
        winapi::VK_NUMPAD1 => Some(VirtualKeyCode::Numpad1),
        winapi::VK_NUMPAD2 => Some(VirtualKeyCode::Numpad2),
        winapi::VK_NUMPAD3 => Some(VirtualKeyCode::Numpad3),
        winapi::VK_NUMPAD4 => Some(VirtualKeyCode::Numpad4),
        winapi::VK_NUMPAD5 => Some(VirtualKeyCode::Numpad5),
        winapi::VK_NUMPAD6 => Some(VirtualKeyCode::Numpad6),
        winapi::VK_NUMPAD7 => Some(VirtualKeyCode::Numpad7),
        winapi::VK_NUMPAD8 => Some(VirtualKeyCode::Numpad8),
        winapi::VK_NUMPAD9 => Some(VirtualKeyCode::Numpad9),
        winapi::VK_MULTIPLY => Some(VirtualKeyCode::NumpadMultiply),
        winapi::VK_ADD => Some(VirtualKeyCode::NumpadPlus),
        //winapi::VK_SEPARATOR => Some(VirtualKeyCode::Separator),
        winapi::VK_SUBTRACT => Some(VirtualKeyCode::NumpadMinus),
        winapi::VK_DECIMAL => Some(VirtualKeyCode::NumpadDecimal),
        winapi::VK_DIVIDE => Some(VirtualKeyCode::NumpadDivide),
        winapi::VK_F1 => Some(VirtualKeyCode::F1),
        winapi::VK_F2 => Some(VirtualKeyCode::F2),
        winapi::VK_F3 => Some(VirtualKeyCode::F3),
        winapi::VK_F4 => Some(VirtualKeyCode::F4),
        winapi::VK_F5 => Some(VirtualKeyCode::F5),
        winapi::VK_F6 => Some(VirtualKeyCode::F6),
        winapi::VK_F7 => Some(VirtualKeyCode::F7),
        winapi::VK_F8 => Some(VirtualKeyCode::F8),
        winapi::VK_F9 => Some(VirtualKeyCode::F9),
        winapi::VK_F10 => Some(VirtualKeyCode::F10),
        winapi::VK_F11 => Some(VirtualKeyCode::F11),
        winapi::VK_F12 => Some(VirtualKeyCode::F12),
        winapi::VK_F13 => Some(VirtualKeyCode::F13),
        winapi::VK_F14 => Some(VirtualKeyCode::F14),
        winapi::VK_F15 => Some(VirtualKeyCode::F15),
        //winapi::VK_F16 => Some(VirtualKeyCode::F16),
        //winapi::VK_F17 => Some(VirtualKeyCode::F17),
        //winapi::VK_F18 => Some(VirtualKeyCode::F18),
        //winapi::VK_F19 => Some(VirtualKeyCode::F19),
        //winapi::VK_F20 => Some(VirtualKeyCode::F20),
        //winapi::VK_F21 => Some(VirtualKeyCode::F21),
        //winapi::VK_F22 => Some(VirtualKeyCode::F22),
        //winapi::VK_F23 => Some(VirtualKeyCode::F23),
        //winapi::VK_F24 => Some(VirtualKeyCode::F24),
        winapi::VK_NUMLOCK => Some(VirtualKeyCode::NumLock),
        winapi::VK_SCROLL => Some(VirtualKeyCode::ScrollLock),
        winapi::VK_BROWSER_BACK => Some(VirtualKeyCode::NavigateBackward),
        winapi::VK_BROWSER_FORWARD => Some(VirtualKeyCode::NavigateForward),
        winapi::VK_BROWSER_REFRESH => Some(VirtualKeyCode::WebRefresh),
        winapi::VK_BROWSER_STOP => Some(VirtualKeyCode::WebStop),
        winapi::VK_BROWSER_SEARCH => Some(VirtualKeyCode::WebSearch),
        winapi::VK_BROWSER_FAVORITES => Some(VirtualKeyCode::WebFavorites),
        winapi::VK_BROWSER_HOME => Some(VirtualKeyCode::WebHome),
        winapi::VK_VOLUME_MUTE => Some(VirtualKeyCode::Mute),
        winapi::VK_VOLUME_DOWN => Some(VirtualKeyCode::VolumeDown),
        winapi::VK_VOLUME_UP => Some(VirtualKeyCode::VolumeUp),
        winapi::VK_MEDIA_NEXT_TRACK => Some(VirtualKeyCode::NextTrack),
        winapi::VK_MEDIA_PREV_TRACK => Some(VirtualKeyCode::PrevTrack),
        winapi::VK_MEDIA_STOP => Some(VirtualKeyCode::MediaStop),
        winapi::VK_MEDIA_PLAY_PAUSE => Some(VirtualKeyCode::PlayPause),
        winapi::VK_LAUNCH_MAIL => Some(VirtualKeyCode::Mail),
        winapi::VK_LAUNCH_MEDIA_SELECT => Some(VirtualKeyCode::MediaSelect),
        //winapi::VK_LAUNCH_APP1 => Some(VirtualKeyCode::Launch_app1),
        //winapi::VK_LAUNCH_APP2 => Some(VirtualKeyCode::Launch_app2),
        //winapi::VK_OEM_PLUS => Some(VirtualKeyCode::Equals),
        //winapi::VK_OEM_COMMA => Some(VirtualKeyCode::Comma),
        //winapi::VK_OEM_MINUS => Some(VirtualKeyCode::Minus),
        //winapi::VK_OEM_PERIOD => Some(VirtualKeyCode::Period),
        //winapi::VK_OEM_1 => map_text_keys(vk),
        //winapi::VK_OEM_2 => map_text_keys(vk),
        //winapi::VK_OEM_3 => map_text_keys(vk),
        //winapi::VK_OEM_4 => map_text_keys(vk),
        //winapi::VK_OEM_5 => map_text_keys(vk),
        //winapi::VK_OEM_6 => map_text_keys(vk),
        //winapi::VK_OEM_7 => map_text_keys(vk),
        //winapi::VK_OEM_8 => Some(VirtualKeyCode::Oem_8),
        //winapi::VK_OEM_102 => Some(VirtualKeyCode::OEM102),
        //winapi::VK_PROCESSKEY => Some(VirtualKeyCode::Processkey),
        //winapi::VK_PACKET => Some(VirtualKeyCode::Packet),
        //winapi::VK_ATTN => Some(VirtualKeyCode::Attn),
        //winapi::VK_CRSEL => Some(VirtualKeyCode::Crsel),
        //winapi::VK_EXSEL => Some(VirtualKeyCode::Exsel),
        //winapi::VK_EREOF => Some(VirtualKeyCode::Ereof),
        //winapi::VK_PLAY => Some(VirtualKeyCode::Play),
        //winapi::VK_ZOOM => Some(VirtualKeyCode::Zoom),
        //winapi::VK_NONAME => Some(VirtualKeyCode::Noname),
        //winapi::VK_PA1 => Some(VirtualKeyCode::Pa1),
        //winapi::VK_OEM_CLEAR => Some(VirtualKeyCode::Oem_clear),
        _ => None
    })
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
    hwnd: winapi::HWND,
    state: WindowState,
    size: Size,
    position: Position,
    context: Context,
    ll: LowLevel,

    view: Rc<RefCell<View>>,
}

impl GLWindow {
    pub fn new(settings: WindowSettings, engine: &Engine, view: Rc<RefCell<View>>) -> Result<Box<GLWindow>, Error> {
        // create OS window
        let engine = engine.platform();
        let app_instance = engine.get_instance();

        let (style, exstyle) = (get_window_style(&settings), get_window_exstyle(&settings));
        let (xpos, ypos) = (winapi::CW_USEDEFAULT, winapi::CW_USEDEFAULT);
        let full_size = get_full_window_size(style, exstyle, settings.size);
        let title = OsStr::new(&settings.title).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
        let hwnd = unsafe {
            user32::CreateWindowExW(exstyle,
                                    engine.get_window_class_name().as_ptr(),
                                    title.as_ptr() as winapi::LPCWSTR,
                                    style,
                                    xpos, ypos,
                                    full_size.width as winapi::LONG, full_size.height as winapi::LONG,
                                    ptr::null_mut(), // No parent window
                                    ptr::null_mut(), // No window menu
                                    app_instance,
                                    ptr::null_mut())
        };
        println!("Os window created: {:?}", hwnd);
        if hwnd.is_null() {
            return Err(Error::WindowCreationError(format!("Window: CreateWindowEx function failed: {}", io::Error::last_os_error())));
        }

        //create context
        let context = match Context::new(app_instance, hwnd, &settings) {
            Err(err) => {
                unsafe {
                    user32::DestroyWindow(hwnd);
                }
                return Err(err);
            }

            Ok(context) => { context }
        };

        let data = Box::new(GLWindow {
            hwnd: hwnd,
            state: WindowState::WaitingOpen,
            size: settings.size,
            position: Position { x: 0, y: 0 },
            context: context,
            ll: LowLevel::new(),

            view: view,
        });

        //connect the OS and rust window
        unsafe {
            let win_ptr = data.as_ref() as *const GLWindow;
            user32::SetWindowLongPtrW(hwnd, 0, win_ptr as i64);
        }

        // ready to show the window
        unsafe {
            user32::ShowWindow(hwnd, winapi::SW_SHOW);

            // The native window creation completes before our callback is injected into the system,
            // thus  a delayed message is sent, that is delivered only when the message loop
            // has started. (engine.dispatch_events)
            user32::PostMessageW(hwnd, WM_DR_WINDOW_CREATED, 0, 0);
        }

        Ok(data)
    }

    pub fn close(&mut self) {
        unsafe {
            user32::PostMessageW(self.hwnd, winapi::WM_CLOSE, 0, 0);
        }
    }

    pub fn is_read_to_render(&self) -> bool {
        self.state == WindowState::Open
    }

    pub fn is_closed(&self) -> bool {
        self.state == WindowState::WaitingClose
            || self.state == WindowState::Closed
            || self.hwnd == ptr::null_mut()
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_draw_size(&self) -> Size {
        self.ll.get_screen_size()
    }

    pub fn start_render(&mut self) -> Result<(), Error> {
        try!(self.context.make_current());
        self.ll.start_render();
        Ok(())
    }

    pub fn end_render(&mut self) -> Result<(), Error> {
        self.ll.end_render();
        try!(self.context.swap_buffers());
        Ok(())
    }

    pub fn get_hwnd(&self) -> winapi::HWND {
        self.hwnd
    }

    pub fn get_ll(&mut self) -> &mut LowLevel {
        &mut self.ll
    }

    /// Static function to handle os messages.
    ///
    /// It converts the raw pointer associated to the OS window back into a safe rust structure.
    pub fn handle_os_message(win_ptr: winapi::LONG_PTR, hwnd: winapi::HWND,
                             msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                             -> winapi::LRESULT {
        let (mut this, win) = unsafe {
            assert!(win_ptr != 0);
            let win_ptr = win_ptr as *mut GLWindow;
            let win = &mut (*win_ptr);
            (TmpWindow(&mut *win_ptr), win)
        };
        let mut result: Option<winapi::LRESULT> = None;

        match msg {
            WM_DR_WINDOW_CREATED => {
                win.state = WindowState::Open;
                if win.start_render().is_ok() {
                    win.view.borrow_mut().on_surface_ready(&mut this);
                    win.end_render().unwrap();
                }
            }

            winapi::WM_CLOSE => {
                win.state = WindowState::WaitingClose;
                if win.start_render().is_ok() {
                    win.view.borrow_mut().on_surface_lost(&mut this);
                    win.end_render().unwrap();
                }
            }

            winapi::WM_DESTROY => {
                win.state = WindowState::Closed;
                unsafe {
                    user32::PostMessageW(ptr::null_mut(), WM_DR_WINDOW_DESTROYED, 0, 0);
                }
            }

            winapi::WM_SIZE => {
                let w = winapi::LOWORD(lparam as winapi::DWORD) as i32;
                let h = winapi::HIWORD(lparam as winapi::DWORD) as i32;

                let size = Size { width: w, height: h };

                unsafe {
                    let mut rect = mem::zeroed();
                    user32::GetWindowRect(win.hwnd, &mut rect);

                    win.size = Size {
                        width: rect.right - rect.left,
                        height: rect.bottom - rect.top
                    };
                }
                win.ll.set_screen_size(size);
                if win.start_render().is_ok() {
                    win.view.borrow_mut().on_surface_changed(&mut this);
                    win.end_render().unwrap();
                }

                result = Some(0);
            }

            winapi::WM_MOVE => {
                let x = winapi::LOWORD(lparam as winapi::DWORD) as i32;
                let y = winapi::HIWORD(lparam as winapi::DWORD) as i32;
                win.position = Position { x: x, y: y };
                result = Some(0);
            }

            winapi::WM_KEYDOWN | winapi::WM_SYSKEYDOWN => {
                if msg == winapi::WM_SYSKEYDOWN && wparam as i32 == winapi::VK_F4 {
                    // pass close by F4 key to windows
                    result = None;
                } else {
                    let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                    win.view.borrow_mut().on_key(&mut this, sc, vkey, true);
                    result = Some(0);
                }
            }

            winapi::WM_KEYUP | winapi::WM_SYSKEYUP => {
                if msg == winapi::WM_SYSKEYUP && wparam as i32 == winapi::VK_F4 {
                    // pass close by F4 key
                    result = None;
                } else {
                    let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                    win.view.borrow_mut().on_key(&mut this, sc, vkey, false);
                    result = Some(0);
                }
            }

            _ => {}
        }

        if let Some(res) = result {
            return res;
        }
        unsafe { user32::DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}

impl Drop for GLWindow {
    fn drop(&mut self) {
        println!("GLWindow dropped");

        unsafe {
            if self.hwnd != ptr::null_mut() {
                // the box is released, thus we remove any dangling pointers
                user32::SetWindowLongPtrW(self.hwnd, 0, 0i64);
                user32::DestroyWindow(self.hwnd);
            }
        }
    }
}


/// Temporary window during view callback handling
struct TmpWindow<'tmp>(&'tmp mut GLWindow);

impl<'engine> Window<'engine> for TmpWindow<'engine> {
    fn platform(&self) -> &GLWindow {
        self.0
    }

    fn platform_mut(&mut self) -> &mut GLWindow {
        self.0
    }
}


pub type WindowImpl = GLWindow;

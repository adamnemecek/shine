use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;
use std::io;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::ntdef::{LONG, LPCWSTR};
use winapi::shared::basetsd::*;
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


/// Maps from windows key code to our virtual key codes
fn vkeycode_to_element(wparam: WPARAM, lparam: LPARAM) -> (ScanCode, Option<VirtualKeyCode>) {
    const MAPVK_VSC_TO_VK_EX: u32 = 3;

    let scancode = ((lparam >> 16) & 0xff) as u32;
    let extended = (lparam & 0x01000000) != 0;
    let vk = match wparam as i32 {
        VK_SHIFT => { ffi!(MapVirtualKeyA(scancode, MAPVK_VSC_TO_VK_EX) as i32) }
        VK_CONTROL => { if extended { VK_RCONTROL } else { VK_LCONTROL } }
        VK_MENU => { if extended { VK_RMENU } else { VK_LMENU } }
        other => other
    };

    // VK_* codes are documented here https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    (scancode, match vk {
        //VK_LBUTTON => Some(VirtualKeyCode::Lbutton),
        //VK_RBUTTON => Some(VirtualKeyCode::Rbutton),
        //VK_CANCEL => Some(VirtualKeyCode::Cancel),
        //VK_MBUTTON => Some(VirtualKeyCode::Mbutton),
        //VK_XBUTTON1 => Some(VirtualKeyCode::Xbutton1),
        //VK_XBUTTON2 => Some(VirtualKeyCode::Xbutton2),
        VK_BACK => Some(VirtualKeyCode::Backspace),
        VK_TAB => Some(VirtualKeyCode::Tab),
        //VK_CLEAR => Some(VirtualKeyCode::Clear),
        VK_RETURN => if extended { Some(VirtualKeyCode::NumpadEnter) } else { Some(VirtualKeyCode::Enter) },
        VK_LSHIFT => Some(VirtualKeyCode::LShift),
        VK_RSHIFT => Some(VirtualKeyCode::RShift),
        VK_LCONTROL => Some(VirtualKeyCode::LControl),
        VK_RCONTROL => Some(VirtualKeyCode::RControl),
        VK_LMENU => Some(VirtualKeyCode::LMenu),
        VK_RMENU => Some(VirtualKeyCode::RMenu),
        VK_PAUSE => Some(VirtualKeyCode::Pause),
        VK_CAPITAL => Some(VirtualKeyCode::CapsLock),
        //VK_KANA => Some(VirtualKeyCode::Kana),
        //VK_HANGUEL => Some(VirtualKeyCode::Hanguel),
        //VK_HANGUL => Some(VirtualKeyCode::Hangul),
        //VK_JUNJA => Some(VirtualKeyCode::Junja),
        //VK_FINAL => Some(VirtualKeyCode::Final),
        //VK_HANJA => Some(VirtualKeyCode::Hanja),
        //VK_KANJI => Some(VirtualKeyCode::Kanji),
        VK_ESCAPE => Some(VirtualKeyCode::Escape),
        //VK_CONVERT => Some(VirtualKeyCode::Convert),
        //VK_NONCONVERT => Some(VirtualKeyCode::NoConvert),
        //VK_ACCEPT => Some(VirtualKeyCode::Accept),
        //VK_MODECHANGE => Some(VirtualKeyCode::Modechange),
        VK_SPACE => Some(VirtualKeyCode::Space),
        VK_PRIOR => Some(VirtualKeyCode::PageUp),
        VK_NEXT => Some(VirtualKeyCode::PageDown),
        VK_END => Some(VirtualKeyCode::End),
        VK_HOME => Some(VirtualKeyCode::Home),
        VK_LEFT => Some(VirtualKeyCode::Left),
        VK_UP => Some(VirtualKeyCode::Up),
        VK_RIGHT => Some(VirtualKeyCode::Right),
        VK_DOWN => Some(VirtualKeyCode::Down),
        //VK_SELECT => Some(VirtualKeyCode::Select),
        //VK_PRINT => Some(VirtualKeyCode::Print),
        //VK_EXECUTE => Some(VirtualKeyCode::Execute),
        VK_SNAPSHOT => Some(VirtualKeyCode::PrintScreen),
        VK_INSERT => Some(VirtualKeyCode::Insert),
        VK_DELETE => Some(VirtualKeyCode::Delete),
        //VK_HELP => Some(VirtualKeyCode::Help),
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
        //VK_LWIN => Some(VirtualKeyCode::Lwin),
        //VK_RWIN => Some(VirtualKeyCode::Rwin),
        VK_APPS => Some(VirtualKeyCode::Apps),
        VK_SLEEP => Some(VirtualKeyCode::Sleep),
        VK_NUMPAD0 => Some(VirtualKeyCode::Numpad0),
        VK_NUMPAD1 => Some(VirtualKeyCode::Numpad1),
        VK_NUMPAD2 => Some(VirtualKeyCode::Numpad2),
        VK_NUMPAD3 => Some(VirtualKeyCode::Numpad3),
        VK_NUMPAD4 => Some(VirtualKeyCode::Numpad4),
        VK_NUMPAD5 => Some(VirtualKeyCode::Numpad5),
        VK_NUMPAD6 => Some(VirtualKeyCode::Numpad6),
        VK_NUMPAD7 => Some(VirtualKeyCode::Numpad7),
        VK_NUMPAD8 => Some(VirtualKeyCode::Numpad8),
        VK_NUMPAD9 => Some(VirtualKeyCode::Numpad9),
        VK_MULTIPLY => Some(VirtualKeyCode::NumpadMultiply),
        VK_ADD => Some(VirtualKeyCode::NumpadPlus),
        //VK_SEPARATOR => Some(VirtualKeyCode::Separator),
        VK_SUBTRACT => Some(VirtualKeyCode::NumpadMinus),
        VK_DECIMAL => Some(VirtualKeyCode::NumpadDecimal),
        VK_DIVIDE => Some(VirtualKeyCode::NumpadDivide),
        VK_F1 => Some(VirtualKeyCode::F1),
        VK_F2 => Some(VirtualKeyCode::F2),
        VK_F3 => Some(VirtualKeyCode::F3),
        VK_F4 => Some(VirtualKeyCode::F4),
        VK_F5 => Some(VirtualKeyCode::F5),
        VK_F6 => Some(VirtualKeyCode::F6),
        VK_F7 => Some(VirtualKeyCode::F7),
        VK_F8 => Some(VirtualKeyCode::F8),
        VK_F9 => Some(VirtualKeyCode::F9),
        VK_F10 => Some(VirtualKeyCode::F10),
        VK_F11 => Some(VirtualKeyCode::F11),
        VK_F12 => Some(VirtualKeyCode::F12),
        VK_F13 => Some(VirtualKeyCode::F13),
        VK_F14 => Some(VirtualKeyCode::F14),
        VK_F15 => Some(VirtualKeyCode::F15),
        //VK_F16 => Some(VirtualKeyCode::F16),
        //VK_F17 => Some(VirtualKeyCode::F17),
        //VK_F18 => Some(VirtualKeyCode::F18),
        //VK_F19 => Some(VirtualKeyCode::F19),
        //VK_F20 => Some(VirtualKeyCode::F20),
        //VK_F21 => Some(VirtualKeyCode::F21),
        //VK_F22 => Some(VirtualKeyCode::F22),
        //VK_F23 => Some(VirtualKeyCode::F23),
        //VK_F24 => Some(VirtualKeyCode::F24),
        VK_NUMLOCK => Some(VirtualKeyCode::NumLock),
        VK_SCROLL => Some(VirtualKeyCode::ScrollLock),
        VK_BROWSER_BACK => Some(VirtualKeyCode::NavigateBackward),
        VK_BROWSER_FORWARD => Some(VirtualKeyCode::NavigateForward),
        VK_BROWSER_REFRESH => Some(VirtualKeyCode::WebRefresh),
        VK_BROWSER_STOP => Some(VirtualKeyCode::WebStop),
        VK_BROWSER_SEARCH => Some(VirtualKeyCode::WebSearch),
        VK_BROWSER_FAVORITES => Some(VirtualKeyCode::WebFavorites),
        VK_BROWSER_HOME => Some(VirtualKeyCode::WebHome),
        VK_VOLUME_MUTE => Some(VirtualKeyCode::Mute),
        VK_VOLUME_DOWN => Some(VirtualKeyCode::VolumeDown),
        VK_VOLUME_UP => Some(VirtualKeyCode::VolumeUp),
        VK_MEDIA_NEXT_TRACK => Some(VirtualKeyCode::NextTrack),
        VK_MEDIA_PREV_TRACK => Some(VirtualKeyCode::PrevTrack),
        VK_MEDIA_STOP => Some(VirtualKeyCode::MediaStop),
        VK_MEDIA_PLAY_PAUSE => Some(VirtualKeyCode::PlayPause),
        VK_LAUNCH_MAIL => Some(VirtualKeyCode::Mail),
        VK_LAUNCH_MEDIA_SELECT => Some(VirtualKeyCode::MediaSelect),
        //VK_LAUNCH_APP1 => Some(VirtualKeyCode::Launch_app1),
        //VK_LAUNCH_APP2 => Some(VirtualKeyCode::Launch_app2),
        //VK_OEM_PLUS => Some(VirtualKeyCode::Equals),
        //VK_OEM_COMMA => Some(VirtualKeyCode::Comma),
        //VK_OEM_MINUS => Some(VirtualKeyCode::Minus),
        //VK_OEM_PERIOD => Some(VirtualKeyCode::Period),
        //VK_OEM_1 => map_text_keys(vk),
        //VK_OEM_2 => map_text_keys(vk),
        //VK_OEM_3 => map_text_keys(vk),
        //VK_OEM_4 => map_text_keys(vk),
        //VK_OEM_5 => map_text_keys(vk),
        //VK_OEM_6 => map_text_keys(vk),
        //VK_OEM_7 => map_text_keys(vk),
        //VK_OEM_8 => Some(VirtualKeyCode::Oem_8),
        //VK_OEM_102 => Some(VirtualKeyCode::OEM102),
        //VK_PROCESSKEY => Some(VirtualKeyCode::Processkey),
        //VK_PACKET => Some(VirtualKeyCode::Packet),
        //VK_ATTN => Some(VirtualKeyCode::Attn),
        //VK_CRSEL => Some(VirtualKeyCode::Crsel),
        //VK_EXSEL => Some(VirtualKeyCode::Exsel),
        //VK_EREOF => Some(VirtualKeyCode::Ereof),
        //VK_PLAY => Some(VirtualKeyCode::Play),
        //VK_ZOOM => Some(VirtualKeyCode::Zoom),
        //VK_NONAME => Some(VirtualKeyCode::Noname),
        //VK_PA1 => Some(VirtualKeyCode::Pa1),
        //VK_OEM_CLEAR => Some(VirtualKeyCode::Oem_clear),
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

pub struct GLWindowControl {
    hwnd: HWND,
}

impl WindowControl for GLWindowControl {
    fn close(&mut self) {
        ffi!(PostMessageW(self.hwnd, WM_CLOSE, 0, 0));
    }
}

/// Structure to store platform dependent data associated to a Window.
pub struct GLWindow {
    hwnd: HWND,
    state: WindowState,
    size: Size,
    position: Position,
    context: GLContext,
    control: GLWindowControl,
    backend: GLBackend,

    view: Rc<RefCell<View<PlatformEngine>>>,
}

impl GLWindow {
    pub fn new_boxed(settings: &PlatformWindowSettings, engine: &GLEngine, view: Rc<RefCell<View<PlatformEngine>>>) -> Result<Box<GLWindow>, Error> {
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

        let data = Box::new(GLWindow {
            hwnd: hwnd,
            state: WindowState::WaitingOpen,
            size: settings.size,
            position: Position { x: 0, y: 0 },
            context: context,
            control: GLWindowControl { hwnd: hwnd },
            backend: GLBackend::new(),
            view: view,
        });

        //connect the OS and rust window
        {
            let win_ptr = data.as_ref() as *const GLWindow;
            ffi!(SetWindowLongPtrW(hwnd, 0, win_ptr as isize));
        }

        // ready to show the window
        ffi!(ShowWindow(hwnd, SW_SHOW));

        // The native window creation completes before our callback is injected into the system,
        // thus  a delayed message is sent, that is delivered only when the message loop
        // has started. (engine.dispatch_events)
        ffi!(PostMessageW(hwnd, win_messages::WM_DR_WINDOW_CREATED, 0, 0));

        Ok(data)
    }

    pub fn close(&mut self) {
        self.control.close();
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
        self.backend.get_screen_size()
    }

    pub fn update_view(&mut self) {
        // make_current is not required as it there is a single context for each window
        // and it is made current during surface events
        if self.is_ready_to_render() {
            self.view.borrow_mut().on_update(&mut self.control, &mut self.backend);
            self.backend.flush();
        }
    }

    pub fn is_ready_to_render(&self) -> bool {
        self.state == WindowState::Open
    }

    pub fn render(&mut self) -> Result<(), Error> {
        // make_current is not required as it there is a single context for each window
        // and it is made current during surface events

        if self.is_ready_to_render() {
            self.view.borrow_mut().on_render(&mut self.control, &mut self.backend);
            self.backend.flush();
            try!(self.context.swap_buffers());
        }
        Ok(())
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    fn handle_surface_ready(&mut self) {
        if self.context.make_current().is_ok() {
            self.view.borrow_mut().on_surface_ready(&mut self.control, &mut self.backend);
            self.backend.flush();
        }
    }

    fn handle_surface_lost(&mut self) {
        if self.context.make_current().is_ok() {
            self.view.borrow_mut().on_surface_lost(&mut self.control, &mut self.backend);
            self.backend.flush();
        }
    }

    fn handle_surface_changed(&mut self) {
        if self.context.make_current().is_ok() {
            self.view.borrow_mut().on_surface_changed(&mut self.control, &mut self.backend);
            self.backend.flush();
        }
    }

    fn handle_key(&mut self, scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        self.view.borrow_mut().on_key(&mut self.control, scan_code, virtual_key, is_down);
    }

    /// Static function to handle os messages.
    ///
    /// It converts the raw pointer associated to the OS window back into a safe rust structure.
    pub fn handle_os_message(win_ptr: LONG_PTR, hwnd: HWND,
                             msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        assert!(win_ptr != 0);

        // It's an OS callback that requires some dark magic to find the rust window associated to the
        // OS window and hence we are not owning the window.
        // We just pretend that we have an owned window and leak it at the and
        let mut win = unsafe { Box::from_raw(win_ptr as *mut GLWindow) };

        let mut result: Option<LRESULT> = None;
        match msg {
            win_messages::WM_DR_WINDOW_CREATED => {
                win.state = WindowState::Open;
                win.handle_surface_ready();
            }

            WM_CLOSE => {
                win.state = WindowState::WaitingClose;
                win.handle_surface_lost();
            }

            WM_DESTROY => {
                win.state = WindowState::Closed;
                ffi!(PostMessageW(ptr::null_mut(), win_messages::WM_DR_WINDOW_DESTROYED, 0, 0));
            }

            WM_SIZE => {
                let w = LOWORD(lparam as DWORD) as i32;
                let h = HIWORD(lparam as DWORD) as i32;

                let size = Size { width: w, height: h };

                let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                ffi!(GetWindowRect(win.hwnd, &mut rect));

                win.size = Size {
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                };
                win.backend.set_screen_size(size);
                win.handle_surface_changed();

                result = Some(0);
            }

            WM_MOVE => {
                let x = LOWORD(lparam as DWORD) as i32;
                let y = HIWORD(lparam as DWORD) as i32;
                win.position = Position { x: x, y: y };
                result = Some(0);
            }

            WM_KEYDOWN | WM_SYSKEYDOWN => {
                if msg == WM_SYSKEYDOWN && wparam as i32 == VK_F4 {
                    // pass close by F4 key to windows
                    result = None;
                } else {
                    let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                    win.handle_key(sc, vkey, true);
                    result = Some(0);
                }
            }

            WM_KEYUP | WM_SYSKEYUP => {
                if msg == WM_SYSKEYUP && wparam as i32 == VK_F4 {
                    // pass close by F4 key
                    result = None;
                } else {
                    let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                    win.handle_key(sc, vkey, false);
                    result = Some(0);
                }
            }

            _ => {}
        }

        mem::forget(win);
        if let Some(res) = result {
            return res;
        }
        ffi!(DefWindowProcW(hwnd, msg, wparam, lparam))
    }
}

impl Drop for GLWindow {
    fn drop(&mut self) {
        //println!("GLWindow dropped");
        if self.hwnd != ptr::null_mut() {
            // the box is released, thus we remove any dangling pointers
            ffi!(SetWindowLongPtrW(self.hwnd, 0, 0));
            ffi!(DestroyWindow(self.hwnd));
        }
    }
}

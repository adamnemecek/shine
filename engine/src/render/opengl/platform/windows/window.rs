use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;
use std::io;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use render::user32;
use render::winapi;

use render::*;
use render::opengl::context::wgl::Context;


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

pub struct GLWindowWrapper {
    hwnd: winapi::HWND,
    is_closing: bool,
    size: Size,
    position: Position,
    context: Context,
    ll: LowLevel,
    surface_handler: Option<Rc<RefCell<SurfaceEventHandler>>>,
    input_handler: Option<Rc<RefCell<InputEventHandler>>>,
}

impl Drop for GLWindowWrapper {
    fn drop(&mut self) {
        println!("GLWindowWrapper dropped");
        unsafe {
            user32::DestroyWindow(self.hwnd);
        }
    }
}


pub struct GLWindow(Rc<RefCell<GLWindowWrapper>>);

impl GLWindow {
    pub fn new(settings: WindowSettings, engine: &mut Engine) -> Result<GLWindow, Error> {
        // create window
        let ref mut engine = engine.platform;
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
        println!("created hwnd {:?}", hwnd);

        if hwnd.is_null() {
            return Err(Error::CreationError(format!("Window: CreateWindowEx function failed: {}", io::Error::last_os_error())));
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

        //todo: position might be not valid

        // create rust window
        let win = Rc::new(RefCell::new(GLWindowWrapper {
            hwnd: hwnd,
            is_closing: false,
            size: settings.size,
            position: Position { x: 0, y: 0 },
            surface_handler: None,
            input_handler: None,
            context: context,
            ll: LowLevel::new(),
        }));

        //connect the OS and rust window
        unsafe {
            // the connection string is same as the window class name for simplicity
            let win_ptr = Rc::into_raw(win.clone());
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

        Ok(GLWindow(win))
    }

    pub unsafe fn new_from_raw(ptr: winapi::LONG_PTR) -> GLWindow {
        assert! (ptr != 0);
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

    pub fn set_input_handler<H: InputEventHandler>(&mut self, handler: H) {
        let ref mut window = self.0.borrow_mut();
        window.input_handler = Some(Rc::new(RefCell::new(handler)));
    }

    pub fn close(&mut self) {
        if self.is_closed() {
            return;
        }

        let ref window = self.0.borrow();
        unsafe {
            user32::PostMessageW(window.hwnd, winapi::WM_CLOSE, 0, 0);
        }
    }

    pub fn is_closed(&self) -> bool {
        let ref window = self.0.borrow();
        window.is_closing || window.hwnd == ptr::null_mut()
    }

    pub fn get_position(&self) -> Position {
        let ref window = self.0.borrow();
        window.position
    }

    pub fn get_size(&self) -> Size {
        let ref window = self.0.borrow();
        window.size
    }

    pub fn get_draw_size(&self) -> Size {
        Size { width: 0, height: 0 }
    }

    pub fn start_render(&self) -> Result<(), Error> {
        let ref window = self.0.borrow();
        window.context.make_current()
    }

    pub fn hello_world(&self, t: f32) {
        unsafe {
            gl::ClearColor(t, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), Error> {
        let queue = &mut queue.platform;
        let ref mut window = self.0.borrow_mut();

        for ref mut cmd in queue.iter_mut() {
            cmd.process(&mut window.ll);
        }

        queue.clear();
        Ok(())
    }

    pub fn end_render(&self) -> Result<(), Error> {
        let ref window = self.0.borrow();
        window.context.swap_buffers()
    }

    pub fn get_hwnd(&self) -> winapi::HWND {
        let ref window = self.0.borrow();
        window.hwnd
    }

    pub fn handle_os_message(&mut self, hwnd: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                             -> winapi::LRESULT {
        {
            let ref window = self.0.borrow();
            assert! (window.hwnd == hwnd);
        }

        let mut result: Option<winapi::LRESULT> = None;
        {
            match msg {
                WM_DR_WINDOW_CREATED if !self.is_closed() => {
                    let handler = self.0.borrow().surface_handler.clone();

                    if let Some(ref handler) = handler {
                        handler.borrow_mut().on_ready(&mut self.to_window());
                    }
                }

                winapi::WM_CLOSE => {
                    let handler = {
                        let ref mut window = self.0.borrow_mut();
                        window.is_closing = true;
                        window.surface_handler.clone()
                    };

                    if let Some(ref handler) = handler {
                        handler.borrow_mut().on_lost(&mut self.to_window());
                    }
                }

                winapi::WM_DESTROY => {
                    unsafe {
                        user32::PostMessageW(ptr::null_mut(), WM_DR_WINDOW_DESTROYED, 0, 0);
                    }
                }

                winapi::WM_SIZE => {
                    let handler = self.0.borrow().surface_handler.clone();
                    let w = winapi::LOWORD(lparam as winapi::DWORD) as u32;
                    let h = winapi::HIWORD(lparam as winapi::DWORD) as u32;

                    let size = Size { width: w, height: h };
                    println!("size: {:?}", size);
                    self.0.borrow_mut().size = size;

                    if let Some(ref handler) = handler {
                        handler.borrow_mut().on_changed(&mut self.to_window());
                    }

                    result = Some(0);
                }

                winapi::WM_MOVE => {
                    let x = winapi::LOWORD(lparam as winapi::DWORD) as i32;
                    let y = winapi::HIWORD(lparam as winapi::DWORD) as i32;

                    let pos = Position { x: x, y: y };
                    println!("pos: {:?}", pos);
                    self.0.borrow_mut().position = Position { x: x, y: y };

                    result = Some(0);
                }

                winapi::WM_KEYDOWN | winapi::WM_SYSKEYDOWN => {
                    if msg == winapi::WM_SYSKEYDOWN && wparam as i32 == winapi::VK_F4 {
                        // pass close by F4 key to windows
                        result = None;
                    } else {
                        let handler = self.0.borrow().input_handler.clone();
                        
                        if let Some(ref handler) = handler {
                            let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                            handler.borrow_mut().on_key(&mut self.to_window(), sc, vkey, true);
                        }
                        result = Some(0);
                    }
                }

                winapi::WM_KEYUP | winapi::WM_SYSKEYUP => {
                    if msg == winapi::WM_SYSKEYUP && wparam as i32 == winapi::VK_F4 {
                        // pass close by F4 key
                        result = None;
                    } else {
                        let handler = self.0.borrow().input_handler.clone();

                        if let Some(ref handler) = handler {
                            let (sc, vkey) = vkeycode_to_element(wparam, lparam);
                            handler.borrow_mut().on_key(&mut self.to_window(), sc, vkey, false);
                        }
                        result = Some(0);
                    }
                }

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
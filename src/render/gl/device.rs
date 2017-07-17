extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use render::{IEngine, IWindow};
use render::{EngineFeatures, EngineError, WindowError};
use render::gl::lowlevel::LowLevel;
use render::gl::device::glutin::GlContext;

pub struct GLWindowImpl
{
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    ll: LowLevel,
}

impl GLWindowImpl {
    fn new<T: Into<String>>(width: u32, height: u32, title: T) -> Result<GLWindowImpl, EngineError> {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(true);

        match glutin::GlWindow::new(window_builder, context_builder, &events_loop) {
            Err(glutin::CreationError::OsError(str)) => Err(EngineError::OsError(str)),
            Err(glutin::CreationError::RobustnessNotSupported) => Err(EngineError::FeatureNotSupported(EngineFeatures::Robustness)),
            Err(glutin::CreationError::OpenGlVersionNotSupported) => Err(EngineError::VersionNotSupported),
            Err(glutin::CreationError::NoAvailablePixelFormat) => Err(EngineError::NoAvailableFormat),
            Err(_) => Err(EngineError::Unknown),
            Ok(win) => Ok(GLWindowImpl {
                events_loop: events_loop,
                window: win,
                ll: LowLevel::new()
            }),
        }
    }

    fn close(&mut self) {
        println!("closing a window impl");
        self.ll.close();
        self.window.hide();
    }

    fn get_window_id(&self) -> glutin::WindowId {
        self.window.id()
    }

    fn set_title(&mut self, title: &str) -> Result<(), WindowError> {
        self.window.set_title(title);
        Ok(())
    }

    fn handle_message(&mut self) -> bool {
        let mut is_running = true;
        let my_window_id = self.get_window_id();

        self.events_loop.poll_events(|event|
            match event {
                glutin::Event::WindowEvent { event, window_id } => {
                    assert_eq! (window_id, my_window_id);
                    match event {
                        glutin::WindowEvent::Closed => {
                            is_running = false;
                        }
                        _ => (),
                    }
                }
                _ => ()
                //glutin::Event::Awakened => {}
            });

        is_running
    }

    fn make_current(&mut self) -> Result<(), WindowError> {
        match unsafe { self.window.make_current() } {
            Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
            Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
            //Err(_) => WindowError::Unknown,
            Ok(_) => Ok(())
        }
    }

    fn swap_buffers(&mut self) -> Result<(), WindowError> {
        match self.window.swap_buffers() {
            Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
            Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
            //Err(_) => WindowError::Unknown,
            Ok(_) => Ok(()),
        }
    }

    fn init_gl_functions(&mut self) -> Result<(), WindowError> {
        match self.make_current() {
            Err(e) => Err(e),
            Ok(_) => {
                gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
                Ok(())
            }
        }
    }
}


pub struct GLWindow(Rc<RefCell<Option<GLWindowImpl>>>);

impl Deref for GLWindow {
    type Target = Rc<RefCell<Option<GLWindowImpl>>>;

    fn deref(&self) -> &Rc<RefCell<Option<GLWindowImpl>>> {
        &self.0
    }
}

impl DerefMut for GLWindow {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<Option<GLWindowImpl>>> {
        &mut self.0
    }
}

impl IWindow for GLWindow {
    fn is_closed(&self) -> bool {
        self.borrow().is_none()
    }

    fn close(&mut self) {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.close();
        }
        *self.borrow_mut() = None;
    }

    fn set_title(&mut self, title: &str) -> Result<(), WindowError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.set_title(&title)
        } else {
            Err(WindowError::ContextLost)
        }
    }

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());
        let result = if let Some(ref mut win) = *self.borrow_mut() {
            win.handle_message(/*timeout*/)
        } else {
            false
        };

        if !result {
            self.close();
        }
        result
    }

    fn render_start(&mut self) -> Result<(), WindowError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.make_current()
        } else {
            Err(WindowError::ContextLost)
        }
    }

    fn render_end(&mut self) -> Result<(), WindowError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.swap_buffers()
        } else {
            Err(WindowError::ContextLost)
        }
    }
}


pub struct GLEngineImpl {
    is_gl_initialized: bool,
    windows: Vec<Weak<RefCell<Option<GLWindowImpl>>>>,
}

impl GLEngineImpl {
    fn new() -> GLEngineImpl {
        GLEngineImpl {
            is_gl_initialized: false,
            windows: vec!(),
        }
    }

    fn remove_closed_windows(&mut self) {
        self.windows.retain(|weak_win| {
            if let Some(rc_win) = weak_win.upgrade() {
                println!("can remove: {}", rc_win.borrow().is_none());
                rc_win.borrow().is_none()
            } else {
                false
            }
        });
    }

    fn close_all_windows(&mut self) {
        println!("closing all windows");
        for win in self.windows.iter_mut() {
            println!("converting weak ptr");
            if let Some(rc_win) = win.upgrade() {
                println!("checking for closed window");
                if let Some(ref mut win) = *rc_win.borrow_mut() {
                    println!("closing an open window");
                    win.close();
                }
                *rc_win.borrow_mut() = None;
            }
        }
        self.remove_closed_windows();
    }
}

impl Drop for GLEngineImpl {
    fn drop(&mut self) {
        self.close_all_windows();
    }
}


pub struct GLEngine(Rc<RefCell<GLEngineImpl>>);

impl Deref for GLEngine {
    type Target = Rc<RefCell<GLEngineImpl>>;

    fn deref(&self) -> &Rc<RefCell<GLEngineImpl>> {
        &self.0
    }
}

impl DerefMut for GLEngine {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<GLEngineImpl>> {
        &mut self.0
    }
}

impl IEngine for GLEngine {
    type Window = GLWindow;

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<GLWindow, EngineError> {
        self.borrow_mut().remove_closed_windows();

        match GLWindowImpl::new(width, height, title) {
            Err(e) => Err(e),
            Ok(mut window) =>
                match if self.borrow().is_gl_initialized { window.make_current() } else { window.init_gl_functions() } {
                    Err(e) => Err(EngineError::WindowCreation(e)),
                    Ok(_) => {
                        let rc_window = Rc::new(RefCell::new(Some(window)));
                        self.borrow_mut().windows.push(Rc::downgrade(&rc_window));
                        Ok(GLWindow(rc_window))
                    }
                }
        }
    }

    fn close_all_windows(&mut self) {
        self.borrow_mut().close_all_windows();
    }
}


pub fn create_engine() -> Result<GLEngine, EngineError> {
    Ok(GLEngine(Rc::new(RefCell::new(GLEngineImpl::new()))))
}

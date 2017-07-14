extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use render::{IEngine, IWindow};
use render::{EngineFeatures, EngineError, WindowError};

use render::gl::lowlevel::LowLevel;

struct GLWindowAndLowLevel(glutin::Window, LowLevel);

pub struct GLWindowImpl {
    window: Option<GLWindowAndLowLevel>,
    event_loop: glutin::EventsLoop,
}

impl GLWindowImpl {
    fn new<T: Into<String>>(width: u32, height: u32, title: T) -> Result<GLWindowImpl, EngineError> {
        let event_loop = glutin::EventsLoop::new();
        match glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height)
            .with_vsync()
            .build(&event_loop)
            {
                Err(glutin::CreationError::OsError(str)) => Err(EngineError::OsError(str)),
                Err(glutin::CreationError::RobustnessNotSupported) => Err(EngineError::FeatureNotSupported(EngineFeatures::Robustness)),
                Err(glutin::CreationError::OpenGlVersionNotSupported) => Err(EngineError::VersionNotSupported),
                Err(glutin::CreationError::NoAvailablePixelFormat) => Err(EngineError::NoAvailableFormat),
                Err(_) => Err(EngineError::Unknown),
                Ok(win) => Ok(GLWindowImpl {
                    window: Some(GLWindowAndLowLevel(win, LowLevel::new())),
                    event_loop: event_loop,
                })
            }
    }

    fn mut_window(&mut self) -> &glutin::Window {
        &self.window.as_ref().as_mut().expect("window already closed").0
    }

    fn get_window(&self) -> &glutin::Window {
        &self.window.as_ref().expect("window already closed").0
    }

    #[allow(dead_code)]
    fn mut_ll(&mut self) -> &LowLevel {
      &self.window.as_ref().as_mut().expect("window already closed").1
    }

    #[allow(dead_code)]
    fn get_ll(&self) -> &LowLevel {
        &self.window.as_ref().expect("window already closed").1
    }

    fn is_closed(&self) -> bool {
        self.window.is_none()
    }

    fn close(&mut self) {
        if !self.is_closed() {
            println!("closing a window impl");
            //self.window.as_ref().map(|GLWindowAndLowLevel(w,ll)| w.hide());
            self.window = None;
        }
    }

    fn get_id(&self) -> glutin::WindowId {
        self.get_window().id()
    }

    fn set_title(&mut self, title: &str) -> Result<(), WindowError> {
        if !self.is_closed() {
            self.mut_window().set_title(title);
            Ok(())
        } else {
            Err(WindowError::ContextLost)
        }
    }

    fn make_current(&mut self) -> Result<(), WindowError> {
        if !self.is_closed() {
            match unsafe { self.mut_window().make_current() } {
                Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
                Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
                //Err(_) => WindowError::Unknown,
                Ok(_) => Ok(())
            }
        } else {
            Err(WindowError::ContextLost)
        }
    }

    fn swap_buffers(&mut self) -> Result<(), WindowError> {
        if !self.is_closed() {
            match self.mut_window().swap_buffers() {
                Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
                Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
                //Err(_) => WindowError::Unknown,
                Ok(_) => Ok(()),
            }
        } else {
            Err(WindowError::ContextLost)
        }
    }

    fn init_gl_functions(&mut self) -> Result<(), WindowError> {
        match self.make_current() {
            Err(e) => Err(e),
            Ok(_) => {
                gl::load_with(|symbol| self.mut_window().get_proc_address(symbol) as *const _);
                Ok(())
            }
        }
    }
}


pub struct GLWindow(Rc<RefCell<GLWindowImpl>>);

impl Deref for GLWindow {
    type Target = Rc<RefCell<GLWindowImpl>>;

    fn deref(&self) -> &Rc<RefCell<GLWindowImpl>> {
        &self.0
    }
}

impl DerefMut for GLWindow {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<GLWindowImpl>> {
        &mut self.0
    }
}

impl IWindow for GLWindow {
    fn is_closed(&self) -> bool {
        self.borrow().is_closed()
    }

    fn close(&mut self) {
        self.borrow_mut().close();
    }

    fn set_title(&mut self, title: &str) -> Result<(), WindowError> {
        self.borrow_mut().set_title(&title)
    }

    #[allow(unused_variables)]
    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        if self.is_closed() {
            return false;
        }

        let mut is_closing = false;
        let my_window_id = self.borrow().get_id();

        {
            self.borrow().event_loop.poll_events(|event|
                match event {
                    glutin::Event::WindowEvent { event, window_id } => {
                        assert_eq! (window_id, my_window_id);
                        match event {
                            glutin::WindowEvent::Closed => {
                                is_closing = true;
                            }
                            _ => (),
                        }
                    }
                });
        }

        if is_closing {
            self.close();
            false
        } else {
            true
        }
    }

    fn render_start(&mut self) -> Result<(), WindowError> {
        self.borrow_mut().make_current()
    }

    fn render_end(&mut self) -> Result<(), WindowError> {
        self.borrow_mut().swap_buffers()
    }
}


pub struct GLEngineImpl {
    is_gl_initialized: bool,
    windows: Vec<Weak<RefCell<GLWindowImpl>>>,
}

impl GLEngineImpl {
    fn new() -> GLEngineImpl {
        GLEngineImpl {
            is_gl_initialized: false,
            windows: vec!(),
        }
    }

    fn close_all_windows(&mut self) {
        println!("closing all windows");
        for win in self.windows.iter_mut() {
            println!("converting weak ptr");
            if let Some(rc_win) = win.upgrade() {
                println!("closing an existing window");
                rc_win.borrow_mut().close();
            }
        }

    }
}

impl Drop for GLEngineImpl {
    fn drop(&mut self) {
        println!("closing render");
        for win in self.windows.iter_mut() {
            println!("check weak ptr");
            if let Some(rc_win) = win.upgrade() {
                println!("closing an existing window");
                rc_win.borrow_mut().close();
            }
        }
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
        match GLWindowImpl::new(width, height, title) {
            Err(e) => Err(e),
            Ok(mut window) =>
                match if self.borrow().is_gl_initialized { window.make_current() } else { window.init_gl_functions() } {
                    Err(e) => Err(EngineError::WindowCreation(e)),
                    Ok(_) => {
                        let rc_window = Rc::new(RefCell::new(window));
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

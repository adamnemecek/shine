extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};

use render::{IEngine, EngineFeatures, IWindow, ContextError};
use render::{ISurfaceHandler};

use self::glutin::GlContext;
use render::gl::lowlevel::LowLevel;


impl From<glutin::ContextError> for ContextError {
    fn from(error: glutin::ContextError) -> ContextError {
        match error {
            glutin::ContextError::IoError(ioe) => ContextError::IoError(ioe),
            glutin::ContextError::ContextLost => ContextError::ContextLost,
            //_ => ContextError::Unknown,
        }
    }
}

impl From<glutin::CreationError> for ContextError {
    fn from(error: glutin::CreationError) -> ContextError {
        match error {
            glutin::CreationError::OsError(str) => ContextError::OsError(str),
            glutin::CreationError::RobustnessNotSupported => ContextError::FeatureNotSupported(EngineFeatures::Robustness),
            glutin::CreationError::OpenGlVersionNotSupported => ContextError::VersionNotSupported,
            glutin::CreationError::NoAvailablePixelFormat => ContextError::NoAvailableFormat,
            _ => ContextError::Unknown,
        }
    }
}


pub struct GLWindowImpl
{
    glutin_window: glutin::GlWindow,
    ll: LowLevel,
}

impl GLWindowImpl {
    pub fn new<T: Into<String>>(events_loop: &glutin::EventsLoop, width: u32, height: u32, title: T, init_gl: bool) -> Result<GLWindowImpl, ContextError> {
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(true);

        let glutin_window = try!(glutin::GlWindow::new(window_builder, context_builder, &events_loop));

        if init_gl {
            unsafe {
                try!(glutin_window.make_current());
            }
        } else {
            unsafe {
                try!(glutin_window.make_current());
                gl::load_with(|symbol| glutin_window.get_proc_address(symbol) as *const _);
            }
        }

        Ok(GLWindowImpl {
            events_loop: events_loop,
            glutin_window: glutin_window,
            ll: LowLevel::new()
        })
    }

    fn release(&mut self) {
        let is_current = unsafe { self.glutin_window.make_current() }.is_ok();
        if is_current {
            self.ll.release();
        }
        self.glutin_window.hide();
    }
}


pub struct Window {
    imp: Rc<RefCell<Option<GLWindowImpl>>>,
    surface_handler: Option<Rc<RefCell<ISurfaceHandler>>>,
    trigger_surface_ready: bool,
}

impl Window {
    pub fn new<T: Into<String>>(events_loop: &glutin::EventsLoop, width: u32, height: u32, title: T, init_gl: bool) -> Result<Window, ContextError> {
        let imp = try!(GLWindowImpl::new(&events_loop, width, height, title, init_gl));

        Ok(Window {
            imp: Rc::new(RefCell::new(Some(imp))),
            surface_handler: None,
            trigger_surface_ready: true,
        })
    }

    pub fn render_process<F: FnMut(&mut LowLevel)>(&mut self, mut fun: F) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.imp.borrow_mut() {
            fun(&mut win.ll);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }
}

impl IWindow for Window {
    fn close(&mut self) {
        *self.imp.borrow_mut() = None;
    }

    fn is_closed(&self) -> bool {
        self.imp.borrow().is_none()
    }

    fn set_title(&mut self, title: &str) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.imp.borrow_mut() {
            win.glutin_window.set_title(title);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    fn set_surface_handler<H: ISurfaceHandler>(&mut self, handler: H) {
        self.surface_handler = Some(Rc::new(RefCell::new(handler)));
    }

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());

        // hack to emulate create events
        if self.trigger_surface_ready && self.imp.borrow().is_some() && self.surface_handler.is_some() {
            if let Some(ref mut handler) = self.surface_handler.clone() {
                handler.borrow_mut().on_ready(self);
                self.trigger_surface_ready = false;
            }
        }

        let mut event_list = Vec::new();

        if let Some(ref mut win) = *self.imp.borrow_mut() {
            let my_window_id = win.glutin_window.id();
            win.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent { event, window_id } = event {
                    assert_eq! (window_id, my_window_id);
                    event_list.push(event);
                }
            });
        }

        for event in event_list.into_iter() {
            match event {
                glutin::WindowEvent::Closed => {
                    println!("closed!!!");
                    if let Some(ref mut handler) = self.surface_handler.clone() {
                        handler.borrow_mut().on_lost(self);
                    }
                    if let Some(ref mut win) = *self.imp.borrow_mut() {
                        win.release();
                    }
                    *self.imp.borrow_mut() = None;
                }
                _ => (),
            }
        }

        !self.is_closed()
    }

    fn render_start(&mut self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.imp.borrow_mut() {
            try!(unsafe { win.glutin_window.make_current() });
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    fn render_end(&mut self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.imp.borrow_mut() {
            try!(win.glutin_window.swap_buffers());
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }
}


pub struct GLEngineImpl {
    events_loop: glutin::EventsLoop,
    is_gl_initialized: bool,
    windows: Vec<Weak<RefCell<Option<GLWindowImpl>>>>, // map from winid -> weakptr
}

impl GLEngineImpl {
    fn new() -> GLEngineImpl {
        GLEngineImpl {
            events_loop: glutin::EventsLoop::new(),
            is_gl_initialized: false,
            windows: vec!(),
        }
    }

    fn remove_closed_windows(&mut self) {
        self.windows.retain(|weak_win| {
            if let Some(rc_win) = weak_win.upgrade() {
                rc_win.borrow().is_none()
            } else {
                false
            }
        });
    }

    fn close_all_windows(&mut self) {
        for win in self.windows.iter_mut() {
            if let Some(rc_win) = win.upgrade() {
                *rc_win.borrow_mut()  = None;
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


pub struct Engine(Rc<RefCell<GLEngineImpl>>);

impl Deref for Engine {
    type Target = Rc<RefCell<GLEngineImpl>>;

    fn deref(&self) -> &Rc<RefCell<GLEngineImpl>> {
        &self.0
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<GLEngineImpl>> {
        &mut self.0
    }
}

impl IEngine for Engine {
    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Window, ContextError> {
        self.borrow_mut().remove_closed_windows();

        let window = try!(Window::new(&self.events_loop, width, height, title, self.borrow().is_gl_initialized));
        self.borrow_mut().is_gl_initialized = true;
        self.borrow_mut().windows.push(Rc::downgrade(&window.imp));
        Ok(window)
    }

    fn close_all_windows(&mut self) {
        self.borrow_mut().close_all_windows();
    }

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());

        let mut event_list = Vec::new();

        if let Some(ref mut win) = *self.imp.borrow_mut() {
            let my_window_id = win.glutin_window.id();
            win.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent { event, window_id } = event {
                    assert_eq! (window_id, my_window_id);
                    event_list.push(event);
                }
            });
        }

        for event in event_list.into_iter() {
            match event {
                glutin::WindowEvent::Closed => {
                    println!("closed!!!");
                    if let Some(ref mut handler) = self.surface_handler.clone() {
                        handler.borrow_mut().on_lost(self);
                    }
                    if let Some(ref mut win) = *self.imp.borrow_mut() {
                        win.release();
                    }
                    *self.imp.borrow_mut() = None;
                }
                _ => (),
            }
        }

        !self.is_closed()
    }
}


pub fn create_engine() -> Result<Engine, ContextError> {
    Ok(Engine(Rc::new(RefCell::new(GLEngineImpl::new()))))
}

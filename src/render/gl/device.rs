extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

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


enum PostMassageAction {
    None,
    Remove,
    SurfaceReady,
    SurfaceLost,
}

pub struct GLWindowImpl
{
    glutin_window: glutin::GlWindow,
    ll: LowLevel,

    surface_handler: Option<Rc<RefCell<ISurfaceHandler>>>,
    trigger_surface_ready: bool,
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
            glutin_window: glutin_window,
            ll: LowLevel::new(),
            surface_handler: None,
            trigger_surface_ready: true,
        })
    }

    fn release(&mut self) {
        let is_current = unsafe { self.glutin_window.make_current() }.is_ok();
        if is_current {
            self.ll.release();
        }
        self.glutin_window.hide();
    }

    fn set_title(&mut self, title: &str) {
        self.glutin_window.set_title(title);
    }

    fn set_surface_handler<H: ISurfaceHandler>(&mut self, handler: H) {
        self.surface_handler = Some(Rc::new(RefCell::new(handler)));
    }

    fn handle_message(&mut self, event: glutin::WindowEvent) -> PostMassageAction {
        match event {
            glutin::WindowEvent::Resized(width, height) => {
                if self.trigger_surface_ready {
                    self.trigger_surface_ready = false;
                    return PostMassageAction::SurfaceReady
                }
            }
            glutin::WindowEvent::Suspended(is_suspended) => {
                if is_suspended {
                    return PostMassageAction::SurfaceLost;
                } else {
                    return PostMassageAction::SurfaceReady;
                }
            }
            glutin::WindowEvent::KeyboardInput { .. } => {
                println!("kb input")
            }
            glutin::WindowEvent::Closed => {
                self.release();
                return PostMassageAction::Remove;
            }
            _ => {}
        }
        PostMassageAction::None
    }

    fn render_start(&mut self) -> Result<(), ContextError> {
        try!(unsafe { self.glutin_window.make_current() });
        Ok(())
    }

    fn render_process<F: FnMut(&mut LowLevel)>(&mut self, mut fun: F) -> Result<(), ContextError> {
        fun(&mut self.ll);
        Ok(())
    }

    fn render_end(&mut self) -> Result<(), ContextError> {
        try!(self.glutin_window.swap_buffers());
        Ok(())
    }
}


pub struct Window(Rc<RefCell<Option<GLWindowImpl>>>);

impl Deref for Window {
    type Target = Rc<RefCell<Option<GLWindowImpl>>>;

    fn deref(&self) -> &Rc<RefCell<Option<GLWindowImpl>>> {
        return &self.0
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<Option<GLWindowImpl>>> {
        return &mut self.0
    }
}

impl Window {
    fn handle_message(&mut self, event: glutin::WindowEvent) -> PostMassageAction {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.handle_message(event)
        } else {
            PostMassageAction::None
        }
    }

    pub fn render_process<F: FnMut(&mut LowLevel)>(&mut self, fun: F) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.render_process(fun)
        } else {
            Err(ContextError::ContextLost)
        }
    }
}

impl IWindow for Window {
    fn close(&mut self) {
        *self.borrow_mut() = None;
    }

    fn is_closed(&self) -> bool {
        self.borrow().is_none()
    }

    fn set_title(&mut self, title: &str) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.set_title(title);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    fn set_surface_handler<H: ISurfaceHandler>(&mut self, handler: H) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.set_surface_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    fn render_start(&mut self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.render_start()
        } else {
            Err(ContextError::ContextLost)
        }
    }

    fn render_end(&mut self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.borrow_mut() {
            win.render_end()
        } else {
            Err(ContextError::ContextLost)
        }
    }
}


pub struct GLEngineImpl {
    events_loop: glutin::EventsLoop,
    is_gl_initialized: bool,
    windows: HashMap<glutin::WindowId, Weak<RefCell<Option<GLWindowImpl>>>>,
}

impl GLEngineImpl {
    fn new() -> GLEngineImpl {
        GLEngineImpl {
            events_loop: glutin::EventsLoop::new(),
            is_gl_initialized: false,
            windows: HashMap::new(),
        }
    }

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Window, ContextError> {
        let imp = try!(GLWindowImpl::new(&self.events_loop, width, height, title, self.is_gl_initialized));

        let window_id = imp.glutin_window.id();
        let window = Window(Rc::new(RefCell::new(Some(imp))));

        self.is_gl_initialized = true;
        self.windows.insert(window_id, Rc::downgrade(&window.0));
        Ok(window)
    }

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());

        // collect events
        let mut events = vec!();
        self.events_loop.poll_events(|evt| {
            match evt {
                glutin::Event::WindowEvent { .. } => events.push(evt),
                _ => {}
            }
        });

        // process event
        for event in events.into_iter() {
            if let glutin::Event::WindowEvent { event, window_id } = event {
                // find the window by id
                let mut window;
                let mut surface_handler;
                if let Some(rc_win) = self.windows.get(&window_id).map_or(None, |item| item.upgrade()) {
                    surface_handler = rc_win.borrow().as_ref().map_or(None, |win| win.surface_handler.clone());
                    window = Window(rc_win);
                } else {
                    //some unknown, unhandled window
                    continue;
                }

                // process message
                match window.handle_message(event) {
                    PostMassageAction::Remove => {
                        //todo: shall be triggered by a glutin::WindowEvent::Closing
                        if let Some(ref mut handler) = surface_handler {
                            handler.borrow_mut().on_lost(&mut window);
                        }
                        self.windows.remove(&window_id);
                        println!("window count: {}", self.windows.len());
                    }
                    PostMassageAction::SurfaceReady => {
                        if let Some(ref mut handler) = surface_handler {
                            handler.borrow_mut().on_ready(&mut window);
                        }
                    }
                    PostMassageAction::SurfaceLost => {
                        if let Some(ref mut handler) = surface_handler {
                            handler.borrow_mut().on_lost(&mut window);
                        }
                    }
                    PostMassageAction::None => {}
                }
            }
        }

        !self.windows.is_empty()
    }

    fn close_all_windows(&mut self) {
        /*for win in self.windows.iter_mut() {
            if let Some(rc_win) = win.upgrade() {
                *rc_win.borrow_mut() = None;
            }
        }*/
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
        self.borrow_mut().create_window(width, height, title)
    }

    fn request_quit(&mut self) {
        self.borrow_mut().close_all_windows();
    }

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());
        self.borrow_mut().handle_message(timeout)
    }
}


pub fn create_engine() -> Result<Engine, ContextError> {
    Ok(Engine(Rc::new(RefCell::new(GLEngineImpl::new()))))
}

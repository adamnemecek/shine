extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use render::{IEngine, IWindow};
use render::{EngineFeatures, EngineError, WindowError};

use render::gl::lowlevel::LowLevel;

#[allow(dead_code)]
pub struct GLWindowImpl {
    window: glutin::Window,
    event_loop: glutin::EventsLoop,
    ll: LowLevel,
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
                    window: win,
                    event_loop: event_loop,
                    ll: LowLevel::new()
                })
            }
    }

    fn make_current(&mut self) -> Result<&mut GLWindowImpl, WindowError> {
        match unsafe { self.window.make_current() } {
            Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
            Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
            //Err(_) => WindowError::Unknown,
            Ok(_) => Ok(&mut self)
        }
    }

    fn swap_buffers(&mut self) -> Result<&mut GLWindowImpl, WindowError> {
        match self.window.swap_buffers() {
            Err(glutin::ContextError::IoError(ioe)) => Err(WindowError::IoError(ioe)),
            Err(glutin::ContextError::ContextLost) => Err(WindowError::ContextLost),
            //Err(_) => WindowError::Unknown,
            Ok(_) => Ok(&mut self),
        }
    }

    fn init_gl_functions(&mut self) -> Result<&mut GLWindowImpl, WindowError> {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        Ok(&mut self)
    }
}


pub struct GLWindow {
    win_impl: Rc<RefCell<GLWindow>>,
}
/*
impl Deref for GLWindow {
    type Target = GLWindowImpl;

    fn deref(&self) -> &GLWindowImpl {
        &self.win_impl.borrow()
    }
}

impl DerefMut for GLWindow {
    fn deref_mut(&mut self) -> &mut GLWindowImpl {
        &mut self.win_impl.borrow_mut()
    }
}*/

impl IWindow for GLWindow {
    fn close(&mut self) {
        if self.is_closed() {
            return;
        }
        drop(self.win_impl);
    }

    fn is_closed(&self) -> bool {
        Rc::strong_count(&self.win_impl) > 0
    }

    fn set_title(&mut self, title: &str) -> Result<(), WindowError> {
        Rc::get_mut(&mut self.win_impl).ok_or(WindowError::ContextLost)
            .and_then(|win| {
                win.window.set_title(&title);
                Ok(())
            })
    }

    #[allow(unused_variables)]
    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        if self.is_closed() {
            return false;
        }

        let mut is_closing = false;
        let winId = self.win_impl.id();

        {
            self.event_loop.poll_events(|event|
                match event {
                    glutin::Event::WindowEvent { event, window_id } => {
                        assert_eq! (window_id, winId);
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

    fn render_start(&mut self) -> Result<&mut GLWindow, WindowError> {
        //println!("render_start");
        self.make_current()
    }

    fn render_end(&mut self) -> Result<&mut GLWindow, WindowError> {
        //println!("render_end");
        self.swap_buffers()
    }
}


pub struct GLEngine {
    is_gl_initialized: bool,
    windows: Vec<Weak<RefCell<GLWindow>>>,
}

impl GLEngine {
    fn new() -> Result<GLEngine, EngineError> {
        let engine = GLEngine {
            is_gl_initialized: false,
            windows: vec!(),
        };

        Ok(engine)
    }
}

impl IEngine for GLEngine {
    type Window = GLWindow;

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<GLWindow, EngineError> {
        GLWindowImpl::new(width, height, title).and_then(
            |hwin| {
                // init opengl function
                if let Err(e) = hwin.borrow_mut()
                    .make_current()
                    .and_then(|win| {
                        if !self.is_gl_initialized {
                            win.init_gl_functions()?;
                            self.is_gl_initialized = true;
                        };
                        Ok(win)
                    }) {
                    return Err(EngineError::WindowCreation(e))
                }

                self.windows.push(Rc::downgrade(&hwin));
                Ok(GLWindow { hwin })
            })
    }
}

impl Drop for GLEngine {
    fn drop(&mut self) {
        println!("closing render");
        for hwin in self.windows.iter() {
            println!("closing a window");
            let win = hwin.upgrade().expect("leaking windows");
            win.borrow_mut().close();
        }
    }
}

pub fn create_engine() -> Result<GLEngine, EngineError> {
    GLEngine::new()
}

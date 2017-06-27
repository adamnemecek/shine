extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use render::{RenderEngine, RenderWindow};
use render::{EngineFeatures, EngineError, WindowError};

use render::gl::lowlevel::LowLevel;

#[allow(dead_code)]
pub struct GLWindow {
    window: Option<glutin::Window>,
    event_loop: glutin::EventsLoop,
    ll: LowLevel,
}

pub type GLWindowHandle = Rc<RefCell<GLWindow>>;

impl GLWindow {
    fn new<T: Into<String>>(width: u32, height: u32, title: T) -> Result<GLWindowHandle, EngineError> {
        let event_loop = glutin::EventsLoop::new();
        glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height)
            .with_vsync()
            .build(&event_loop)
            .map_err(|e|
                match e {
                    glutin::CreationError::OsError(str) => EngineError::OsError(str),
                    glutin::CreationError::RobustnessNotSupported => EngineError::FeatureNotSupported(EngineFeatures::Robustness),
                    glutin::CreationError::OpenGlVersionNotSupported => EngineError::VersionNotSupported,
                    glutin::CreationError::NoAvailablePixelFormat => EngineError::NoAvailableFormat,
                    _ => EngineError::Unknown,
                })
            //.map(|window| window.make_current())
            .map(|window|
                Rc::new(RefCell::new(GLWindow {
                    window: Some(window),
                    event_loop: event_loop,
                    ll: LowLevel::new()
                })))
    }

    fn make_current(&mut self) -> Result<&mut GLWindow, WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| unsafe { (*win).make_current() }
                .map_err(|e| (match e {
                    glutin::ContextError::IoError(ioe) => WindowError::IoError(ioe),
                    glutin::ContextError::ContextLost => WindowError::ContextLost,
                    //_ => WindowError::Unknown,
                }))
            )?;

        Ok(self)
    }

    fn swap_buffers(&mut self) -> Result<&mut GLWindow, WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| (*win).swap_buffers()
                .map_err(|e| (match e {
                    glutin::ContextError::IoError(ioe) => WindowError::IoError(ioe),
                    glutin::ContextError::ContextLost => WindowError::ContextLost,
                    //_ => WindowError::Unknown,
                })))?;

        Ok(self)
    }

    fn init_gl_functions(&mut self) -> Result<&mut GLWindow, WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| {
                gl::load_with(|symbol| (*win).get_proc_address(symbol) as *const _);
                Ok(())
            })?;

        Ok(self)
    }
}

impl RenderWindow for GLWindow {
    fn close(&mut self) {
        if self.is_closed() {
            return;
        }
        self.window = None;
    }

    fn is_closed(&self) -> bool {
        self.window.is_none()
    }

    fn set_title(&self, title: &str) -> Result<(), WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| {
                (*win).set_title(&title);
                Ok(())
            })
    }

    #[allow(unused_variables)]
    fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
        if self.window.is_none() {
            return false;
        }

        let mut is_closing = false;

        {
            let win = self.window.as_ref().unwrap();
            self.event_loop.poll_events(|event|
                match event {
                    glutin::Event::WindowEvent { event, window_id } => {
                        assert_eq! (window_id, (*win).id());
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

impl RenderEngine for GLEngine {
    type Window = GLWindow;

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<GLWindowHandle, EngineError> {
        GLWindow::new(width, height, title).and_then(
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
                Ok(hwin)
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

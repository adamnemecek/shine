extern crate glutin;
extern crate gl;

use std::time::{Duration};
use render::{RenderEngine, RenderWindow};
use render::{EngineFeatures, EngineError, WindowError};

pub struct GLWindow {
    window: Option<glutin::Window>,
    event_loop: glutin::EventsLoop,
}

impl GLWindow {
    fn new<T: Into<String>>(width: u32, height: u32, title: T) -> Result<GLWindow, EngineError> {
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
            .map(|window| GLWindow {
                window: Some(window),
                event_loop: event_loop,
            })
    }

    fn make_current(&self) -> Result<(), WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| unsafe { (*win).make_current() }
                .map_err(|e| (match e {
                    glutin::ContextError::IoError(ioe) => WindowError::IoError(ioe),
                    glutin::ContextError::ContextLost => WindowError::ContextLost,
                    //_ => WindowError::Unknown,
                })))
    }

    fn swap_buffers(&self) -> Result<(), WindowError> {
        self.window.as_ref().ok_or(WindowError::ContextLost)
            .and_then(|win| (*win).swap_buffers()
                .map_err(|e| (match e {
                    glutin::ContextError::IoError(ioe) => WindowError::IoError(ioe),
                    glutin::ContextError::ContextLost => WindowError::ContextLost,
                    //_ => WindowError::Unknown,
                })))
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
    fn handle_message(&mut self, timeout: Option<Duration>) {
        if self.window.is_none() {
            return;
        }

        let mut is_closing = false;

        {
            let win = self.window.as_ref().unwrap();
            self.event_loop.poll_events(|event|
                match event {
                    glutin::Event::WindowEvent { event, window_id } => {
                        assert_eq!(window_id, (*win).id());
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
        }
    }

    fn render_start(&self) -> Result<(), WindowError> {
        println!("render_start");
        self.make_current()
    }

    fn render_end(&self) -> Result<(), WindowError> {
        println!("render_end");
        self.swap_buffers()
    }
}


pub struct GLEngine {
    is_gl_initialized: bool,
}

impl RenderEngine for GLEngine {
    type Window = GLWindow;

    fn new() -> Result<GLEngine, ()> {
        let engine = GLEngine {
            is_gl_initialized: false,
        };

        Ok(engine)
    }

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Self::Window, EngineError> {
        GLWindow::new(width, height, title)
            .and_then(|win| match win.make_current() {
                Ok(_) => Ok(win),
                Err(e) => Err(EngineError::WindowCreation(e))
            })
            .and_then(|win| {
                if !self.is_gl_initialized {
                    let glutin_win = win.window.as_ref().unwrap();
                    gl::load_with(|symbol| (*glutin_win).get_proc_address(symbol) as *const _);
                    self.is_gl_initialized = true;
                }
                Ok(win)
            })
    }
}

impl Drop for GLEngine {
    fn drop(&mut self) {
        println!("closing render");
    }
}

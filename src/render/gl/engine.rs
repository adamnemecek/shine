extern crate glutin;
extern crate gl;

use std::time::Duration;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::collections::HashMap;

use render::*;
use render::gl::*;
use render::gl::window::*;
use self::glutin::GlContext;

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

pub struct GLEngine {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Weak<RefCell<Option<GLWindow>>>>,
}

impl GLEngine {
    pub fn new() -> GLEngine {
        GLEngine {
            events_loop: glutin::EventsLoop::new(),
            windows: HashMap::new(),
        }
    }

    pub fn store_window(&mut self, id: glutin::WindowId, window_ref: WeakGLWindow) {
        self.windows.insert(id, window_ref);
    }

    pub fn remove_window(&mut self, id: glutin::WindowId) {
        self.windows.remove(&id);
    }

    pub fn get_window_by_id(&self, id: glutin::WindowId) -> Option<RcGLWindow> {
        if let Some(ref item) = self.windows.get(&id) {
            item.upgrade()
        } else {
            None
        }
    }

    pub fn get_events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    pub fn handle_message(&mut self, timeout: Option<Duration>) -> bool {
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
                let window_wrapper;
                let mut remove_window = false;
                if let Some(ref item) = self.windows.get(&window_id) {
                    if let Some(rc_win) = item.upgrade() {
                        // this is an active window
                        window_wrapper = GLWindowWrapper::wrap(rc_win);
                    } else {
                        // this window is controlled by this engine, but has been closed
                        println!("remove_window");
                        remove_window = true;
                        continue;
                    }
                } else {
                    println!("window not found");
                    // this window is not controlled by this engine
                    continue;
                }

                let action = window_wrapper.handle_message(event);
                match action {
                    MessageAction::SurfaceReady(handler) => {
                        if let Some(h) = handler {
                            h.borrow_mut().on_ready(&window_wrapper.as_window());
                        }
                    }

                    MessageAction::SurfaceLost(handler) => {
                        if let Some(h) = handler {
                            h.borrow_mut().on_lost(&window_wrapper.as_window());
                        }
                    }

                    MessageAction::Destroyed(handler) => {
                        if let Some(h) = handler {
                            h.borrow_mut().on_lost(&window_wrapper.as_window());
                        }
                        window_wrapper.close_from_os();
                        remove_window = true;
                    }

                    MessageAction::InputKey(handler) => {
                        if let Some(h) = handler {
                            h.borrow_mut().on_key(&window_wrapper.as_window());
                        }
                    }

                    MessageAction::None => {}
                }


                if remove_window {
                    self.remove_window(window_id);
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

impl Drop for GLEngine {
    fn drop(&mut self) {
        self.close_all_windows();
    }
}


pub type RcGLEngine = Rc<RefCell<GLEngine>>;

pub struct GLEngineWrapper {
    wrapped: RcGLEngine
}

impl GLEngineWrapper {
    pub fn new() -> Result<GLEngineWrapper, ContextError> {
        Ok(GLEngineWrapper { wrapped: Rc::new(RefCell::new(GLEngine::new())) })
    }

    pub fn wrap(wrapped: RcGLEngine) -> GLEngineWrapper {
        GLEngineWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> RcGLEngine {
        self.wrapped.clone()
    }

    pub fn request_quit(&self) {
        self.wrapped.borrow_mut().close_all_windows();
    }

    pub fn handle_message(&self, timeout: Option<Duration>) -> bool {
        assert!(timeout.is_none());
        self.wrapped.borrow_mut().handle_message(timeout)
    }
}

pub type EngineImpl = GLEngineWrapper;
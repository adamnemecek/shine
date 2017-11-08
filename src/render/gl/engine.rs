extern crate glutin;
extern crate gl;

use std::time::Duration;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
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

pub enum PostMassageAction {
    None,
    Remove,
    SurfaceReady,
    SurfaceLost,
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

    pub fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<GLWindowWrapper, ContextError> {
        let (window_id, window, window_ref) = try!(GLWindowWrapper::new(&self.events_loop, width, height, title));
        self.windows.insert(window_id, window_ref);
        Ok(window)
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
                // find the window by id
                let mut window_wrapper;
                if let Some(rc_win) = self.windows.get(&window_id).map_or(None, |item| item.upgrade()) {
                    window_wrapper = GLWindowWrapper::new_from_rc(rc_win);
                } else {
                    //some unknown, unhandled window
                    continue;
                }

                // process message
                match window_wrapper.handle_message(event) {
                    PostMassageAction::Remove => {
                        //if let Some(ref mut handler) = surface_handler {
//                            handler.borrow_mut().on_lost(Window::new_from_impl(window_wrapper));
//                        }

                        //window_wrapper.release();
                        self.windows.remove(&window_id);
                    }

                    PostMassageAction::SurfaceReady => {
//                        if let Some(ref mut handler) = surface_handler {
  //                          handler.borrow_mut().on_ready(Window::new_from_impl(window_wrapper));
    //                    }
                    }

                    PostMassageAction::SurfaceLost => {
      //                  if let Some(ref mut handler) = surface_handler {
      //                      handler.borrow_mut().on_lost(Window::new_from_impl(window_wrapper));
      //                  }
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

impl Drop for GLEngine {
    fn drop(&mut self) {
        self.close_all_windows();
    }
}


pub struct GLEngineWrapper {
    wrapped: Rc<RefCell<GLEngine>>
}

impl GLEngineWrapper {
    pub fn new() -> Result<GLEngineWrapper, ContextError> {
        Ok(GLEngineWrapper { wrapped: Rc::new(RefCell::new(GLEngine::new())) })
    }

    pub fn create_window<T: Into<String>>(&self, width: u32, height: u32, title: T) -> Result<GLWindowWrapper, ContextError> {
        self.wrapped.borrow_mut().create_window(width, height, title)
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
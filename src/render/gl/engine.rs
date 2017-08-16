extern crate glutin;
extern crate gl;

use std::time::Duration;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::collections::HashMap;

use render::*;
//use render::gl::*;
use render::gl::window::*;

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

pub enum PostMessageAction {
    None,
    Remove,
}

pub enum FindWindowResult {
    None,
    Some(GLWindowWrapper),
    Remove,
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

    pub fn store_window(&mut self, id: glutin::WindowId, window_ref: Weak<RefCell<Option<GLWindow>>>) {
        self.windows.insert(id, window_ref);
    }

    pub fn remove_window(&mut self, id: glutin::WindowId) {
        self.windows.remove(&id);
    }

    pub fn get_window_by_id(&mut self, window_id: glutin::WindowId) -> FindWindowResult {
        if let Some(item) = self.windows.get(&window_id) {
            if let Some(rc_win) = item.upgrade() {
                if rc_win.borrow().is_some() {
                    // this is an active window
                    FindWindowResult::Some(GLWindowWrapper::wrap(rc_win))
                } else {
                    // this window was controlled by this engine, but has been closed
                    // maybe we should assert here
                    FindWindowResult::Remove
                }
            } else {
                // this window is not controlled by this engine
                FindWindowResult::None
            }
        } else {
            FindWindowResult::None
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

        // Add an explicit close event as if it was sent by the OS.
        // We release the window and remove from our list before glutin window
        // is dropped (or OS has sent the close event), but it's ok, as all our resources are
        // released.
        for item in self.windows.iter() {
            if let Some(rc_win) = item.1.upgrade() {
                let window_wrapper = GLWindowWrapper::wrap(rc_win);
                if window_wrapper.is_close_requested() {
                    let event = glutin::Event::WindowEvent {
                        window_id: *item.0,
                        event: glutin::WindowEvent::Closed
                    };
                    events.push(event);
                }
            }
        }

        // process event
        for event in events.into_iter() {
            if let glutin::Event::WindowEvent { event, window_id } = event {
                match self.get_window_by_id(window_id) {
                    FindWindowResult::Some(window) => {
                        match window.handle_message(event) {
                            PostMessageAction::Remove => { self.remove_window(window_id); }
                            PostMessageAction::None => {}
                        }
                    }

                    FindWindowResult::Remove => { self.remove_window(window_id); }

                    FindWindowResult::None => {}
                }
            }
        }

        !self.windows.is_empty()
    }

    fn close_all_windows(&mut self) {
        for item in self.windows.iter() {
            if let Some(rc_win) = item.1.upgrade() {
                let window_wrapper = GLWindowWrapper::wrap(rc_win);
                window_wrapper.request_close();
            }
        }
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

    pub fn wrap(wrapped: Rc<RefCell<GLEngine>>) -> GLEngineWrapper {
        GLEngineWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> Rc<RefCell<GLEngine>> {
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
extern crate glutin;
extern crate gl;

use std::time::Duration;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};

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

pub enum FindWindowResult {
    None,
    Some(GLWindowWrapper),
    Remove,
}

pub struct GLEngine {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Weak<RefCell<Option<GLWindow>>>>,
    close_requests: HashSet<glutin::WindowId>,
}

impl GLEngine {
    pub fn new() -> GLEngine {
        GLEngine {
            events_loop: glutin::EventsLoop::new(),
            windows: HashMap::new(),
            close_requests: HashSet::new(),
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

    pub fn request_close(&mut self, window_id: glutin::WindowId) {
        self.close_requests.insert(window_id);
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

        // Add an explicit close event to trigger cleanup.
        // It will also trigger an OS close when the glutin window is dropped,
        // but we have already cleaned up everything by that time. So it shall be ok.
        for closed_item in self.close_requests.iter() {
            let event = glutin::Event::WindowEvent {
                window_id: *closed_item,
                event: glutin::WindowEvent::Closed
            };
            events.push(event);
        }
        self.close_requests.clear();

        // process event
        for event in events.into_iter() {
            if let glutin::Event::WindowEvent { event, window_id } = event {
                let action = match self.get_window_by_id(window_id) {
                    FindWindowResult::Some(window) => { window.handle_message(event) }
                    FindWindowResult::Remove => { MessageAction::Remove }
                    FindWindowResult::None => {}
                };

                match action {
                    MessageAction::SurfaceReady(handler) => {
                        if let Some(h) = handler {
                            let window = self.as_window();
                            h.borrow_mut().on_ready(&window);
                        }
                        PostMessageAction::None
                    }

                    MessageAction::SurfaceLost(handler) => {
                        if let Some(h) = handler {
                            let window = self.as_window();
                            h.borrow_mut().on_lost(&window);
                        }
                        PostMessageAction::None
                    }

                    MessageAction::Destroyed(handler) => {
                        if let Some(h) = handler {
                            let window = self.as_window();
                            h.borrow_mut().on_lost(&window);
                        }
                        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
                            win.release();
                        }
                        PostMessageAction::Remove
                    }

                    MessageAction::InputKey(handler) => {
                        if let Some(h) = handler {
                            let window = self.as_window();
                            h.borrow_mut().on_key(&window);
                        }
                        PostMessageAction::None
                    }

                    MessageAction::None => {
                        PostMessageAction::None
                    }
                }
            }
        }

        !self.windows.is_empty()
    }

    fn close_all_windows(&mut self) {
        for item in self.windows.iter_mut() {
            let window_wrapper = GLWindowWrapper::wrap(item.1.upgrade().unwrap());
            window_wrapper.close();
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
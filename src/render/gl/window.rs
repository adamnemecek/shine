extern crate glutin;
extern crate gl;

use std::time::{Duration};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use render::*;
use render::gl::*;
use render::gl::engine::*;
use self::glutin::GlContext;

pub type SurfaceHandlerWrapper = Rc<RefCell<SurfaceHandler>>;

pub struct GLWindow
{
    glutin_window: glutin::GlWindow,
    ll: LowLevel,

    surface_handler: Option<SurfaceHandlerWrapper>,
    trigger_surface_ready: bool,
}

impl GLWindow {
    pub fn new<T: Into<String>>(events_loop: &glutin::EventsLoop, width: u32, height: u32, title: T) -> Result<GLWindow, ContextError> {
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(true);

        let glutin_window = try!(glutin::GlWindow::new(window_builder, context_builder, &events_loop));

        unsafe {
            try!(glutin_window.make_current());
            gl::load_with(|symbol| glutin_window.get_proc_address(symbol) as *const _);
        }

        Ok(GLWindow {
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

    fn set_surface_handler<S: SurfaceHandler>(&mut self, handler: S) {
        self.surface_handler = Some(Rc::new(RefCell::new(handler)));
    }

    fn get_surface_handler(&self) -> Option<SurfaceHandlerWrapper> {
        self.surface_handler.map(|sh| sh.clone())
    }

    fn handle_message(&mut self, event: glutin::WindowEvent) -> PostMassageAction {
        match event {
            glutin::WindowEvent::Resized(width, height) => {
                if self.trigger_surface_ready {
                    self.trigger_surface_ready = false;
                    return PostMassageAction::SurfaceReady
                }
                self.ll.set_size(width, height);
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
                return PostMassageAction::Remove;
            }
            _ => {}
        }
        PostMassageAction::None
    }

    fn start_render(&mut self) -> Result<(), ContextError> {
        try!(unsafe { self.glutin_window.make_current() });
        self.ll.start_render();
        Ok(())
    }

    /*fn process_queue(&mut self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        for ref mut cmd in queue.iter_mut() {
            cmd.process(&mut self.ll);
        }
        queue.clear();
        Ok(())
    }*/

    fn end_render(&mut self) -> Result<(), ContextError> {
        self.ll.end_render();
        try!(self.glutin_window.swap_buffers());
        Ok(())
    }
}

pub type WeakGLWindow = Weak<RefCell<Option<GLWindow>>>;
pub type RcGLWindow = Rc<RefCell<Option<GLWindow>>>;

pub struct GLWindowWrapper {
    wrapped: RcGLWindow
}

impl GLWindowWrapper {
    pub fn new<T: Into<String>>(events_loop: &glutin::EventsLoop, width: u32, height: u32, title: T)
                                -> Result<(glutin::WindowId, GLWindowWrapper, WeakGLWindow), ContextError> {
        let imp = try!(GLWindow::new(&events_loop, width, height, title));
        let window_id = imp.glutin_window.id();
        let rc_window = Rc::new(RefCell::new(Some(imp)));
        let weak_window = Rc::downgrade(&rc_window);
        Ok((window_id, GLWindowWrapper { wrapped: rc_window }, weak_window))
    }

    pub fn new_from_rc(rc_window: RcGLWindow) -> GLWindowWrapper {
        GLWindowWrapper { wrapped: rc_window }
    }

    pub fn release(&mut self) {
        // This function is used to close the window from the program, and it is also called when the OS
        // event handler handles the close event. As the option is nulled after the first call,
        // double-release is possible.
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.release();
        }
        *self.wrapped.borrow_mut() = None;
    }

    pub fn is_closed(&self) -> bool {
        self.wrapped.borrow().is_none()
    }

    pub fn set_title(&self, title: &str) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_title(title);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn set_surface_handler<S:SurfaceHandler>(&self, handler: S) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_surface_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn get_surface_handler(&self) -> Option<SurfaceHandlerWrapper> {
        if let Some(ref win) = *self.wrapped.borrow() {
            win.get_surface_handler()
        } else {
            None
        }
    }

    pub fn handle_message(&self, event: glutin::WindowEvent) -> PostMassageAction {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.handle_message(event)
        } else {
            PostMassageAction::None
        }
    }

    pub fn start_render(&self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.start_render()
        } else {
            Err(ContextError::ContextLost)
        }
    }

    /*fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.process_queue(queue)
        } else {
            Err(ContextError::ContextLost)
        }
    }*/

    pub fn end_render(&self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.end_render()
        } else {
            Err(ContextError::ContextLost)
        }
    }
}

pub type WindowImpl = GLWindowWrapper;

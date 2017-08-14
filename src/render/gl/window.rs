extern crate glutin;
extern crate gl;

use std::rc::{Rc, Weak};
use std::cell::{RefCell};

use render::*;
use render::gl::*;
use render::gl::engine::*;
use self::glutin::GlContext;

pub type OptionSurfaceHandler = Option<Rc<RefCell<SurfaceHandler>>>;
pub type OptionInputHandler = Option<Rc<RefCell<InputHandler>>>;

//Message handler callbacks cannot be called while the window is borrowed.
// So instead of calling the callbacks directly the parameters are
// collected and called when after the scope of borrowing over
pub enum MessageAction {
    None,
    Destroyed(OptionSurfaceHandler),
    SurfaceReady(OptionSurfaceHandler),
    SurfaceLost(OptionSurfaceHandler),

    InputKey(OptionInputHandler),
}


pub struct GLWindow
{
    glutin_window: glutin::GlWindow,
    ll: LowLevel,

    surface_handler: OptionSurfaceHandler,
    input_handler: OptionInputHandler,
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
            input_handler: None,
            trigger_surface_ready: true,
        })
    }

    fn release(&mut self) {
        println!("win release");
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

    fn set_input_handler<S: InputHandler>(&mut self, handler: S) {
        self.input_handler = Some(Rc::new(RefCell::new(handler)));
    }

    fn handle_message(&mut self, event: glutin::WindowEvent) -> MessageAction {
        match event {
            glutin::WindowEvent::Resized(width, height) => {
                self.ll.set_size(width, height);
                if self.trigger_surface_ready {
                    self.trigger_surface_ready = false;
                    MessageAction::SurfaceReady(self.surface_handler.clone())
                } else {
                    MessageAction::None
                }
            }

            glutin::WindowEvent::Suspended(is_suspended) => {
                if is_suspended {
                    MessageAction::SurfaceLost(self.surface_handler.clone())
                } else {
                    MessageAction::SurfaceReady(self.surface_handler.clone())
                }
            }

            glutin::WindowEvent::KeyboardInput { .. } => {
                println!("kb input");
                MessageAction::InputKey(self.input_handler.clone())
            }

            glutin::WindowEvent::Closed => {
                MessageAction::Destroyed(self.surface_handler.clone())
            }

            _ => MessageAction::None
        }
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
    pub fn new<T: Into<String>>(engine: &GLEngineWrapper, width: u32, height: u32, title: T) -> Result<GLWindowWrapper, ContextError> {
        let e = engine.unwrap();
        let imp = try!(GLWindow::new(e.borrow().get_events_loop(), width, height, title));
        let window_id = imp.glutin_window.id();
        let rc_window = Rc::new(RefCell::new(Some(imp)));
        let weak_window = Rc::downgrade(&rc_window);
        e.borrow_mut().store_window(window_id, weak_window);
        Ok(GLWindowWrapper { wrapped: rc_window })
    }

    pub fn wrap(wrapped: RcGLWindow) -> GLWindowWrapper {
        GLWindowWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> RcGLWindow {
        self.wrapped.clone()
    }

    pub fn as_window(&self) -> Window {
        Window::new_from_impl(GLWindowWrapper { wrapped: self.wrapped.clone() })
    }

    pub fn is_closed(&self) -> bool {
        self.wrapped.borrow().is_none()
    }

    pub fn close(&self) {
        //This function is used in two scenario
        //  - when the window is closed by explicitly calling the close function
        //  - when the OS requested the close (ex by pressing the "close" window button (X)
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.release();
        }
        *self.wrapped.borrow_mut() = None;
    }

    pub fn set_title(&self, title: &str) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_title(title);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn set_surface_handler<S: SurfaceHandler>(&self, handler: S) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_surface_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn set_input_handler<S: InputHandler>(&self, handler: S) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_input_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn handle_message(&self, event: glutin::WindowEvent) -> MessageAction {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.handle_message(event)
        } else {
            MessageAction::None
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

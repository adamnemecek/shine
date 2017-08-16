extern crate glutin;
extern crate gl;

use std::rc::Rc;
use std::cell::RefCell;

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
    is_close_requested: bool,
    glutin_window: glutin::GlWindow,
    ll: LowLevel,

    surface_handler: OptionSurfaceHandler,
    input_handler: OptionInputHandler,
    trigger_surface_ready: bool,
}

impl GLWindow {
    fn new<T: Into<String>>(events_loop: &glutin::EventsLoop, width: u32, height: u32, title: T) -> Result<GLWindow, ContextError> {
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
            is_close_requested: false,
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

    pub fn is_close_requested(&self) -> bool {
        self.is_close_requested
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
        println!("pre make" );
        try!(unsafe { self.glutin_window.make_current() });
        println!("post make" );
        self.ll.start_render();
        Ok(())
    }

    fn process_queue(&mut self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        for ref mut cmd in queue.platform.iter_mut() {
            cmd.process(&mut self.ll);
        }
        queue.platform.clear();
        Ok(())
    }

    fn end_render(&mut self) -> Result<(), ContextError> {
        self.ll.end_render();
        try!(self.glutin_window.swap_buffers());
        Ok(())
    }
}

pub struct GLWindowWrapper {
    wrapped: Rc<RefCell<Option<GLWindow>>>
}

impl GLWindowWrapper {
    pub fn new<T: Into<String>>(engine: &mut Engine, width: u32, height: u32, title: T) -> Result<GLWindowWrapper, ContextError> {
        let engine = &mut engine.platform;
        let imp = try!(GLWindow::new(engine.get_events_loop(), width, height, title));
        let window_id = imp.glutin_window.id();
        let rc_window = Rc::new(RefCell::new(Some(imp)));
        let weak_window = Rc::downgrade(&rc_window);
        engine.store_window(window_id, weak_window);
        Ok(GLWindowWrapper { wrapped: rc_window })
    }

    pub fn wrap(wrapped: Rc<RefCell<Option<GLWindow>>>) -> GLWindowWrapper {
        GLWindowWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<GLWindow>>> {
        self.wrapped.clone()
    }

    pub fn as_window(&self) -> Window {
        Window::new_platform(GLWindowWrapper { wrapped: self.wrapped.clone() })
    }

    pub fn is_close_requested(&self) -> bool {
        if let Some(ref win) = *self.wrapped.borrow() {
            win.is_close_requested
        } else {
            false
        }
    }

    pub fn is_closed(&self) -> bool {
        self.wrapped.borrow().is_none()
    }

    pub fn request_close(&mut self) {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.is_close_requested = true;
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_title(title);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn set_surface_handler<S: SurfaceHandler>(&mut self, handler: S) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_surface_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn set_input_handler<S: InputHandler>(&mut self, handler: S) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.set_input_handler(handler);
            Ok(())
        } else {
            Err(ContextError::ContextLost)
        }
    }

    pub fn handle_message(&mut self, event: glutin::WindowEvent) -> PostMessageAction {
        let action =
            if let Some(ref mut win) = *self.wrapped.borrow_mut() {
                win.handle_message(event)
            } else {
                MessageAction::None
            };

        match action {
            MessageAction::SurfaceReady(handler) => {
                if let Some(h) = handler { h.borrow_mut().on_ready(&mut self.as_window()); }
                PostMessageAction::None
            }

            MessageAction::SurfaceLost(handler) => {
                if let Some(h) = handler { h.borrow_mut().on_lost(&mut self.as_window()); }
                PostMessageAction::None
            }

            MessageAction::Destroyed(handler) => {
                println!("before on_lst" );
                if let Some(h) = handler { h.borrow_mut().on_lost(&mut self.as_window()); }
                println!("after on_lst" );
                if let Some(ref mut win) = *self.wrapped.borrow_mut() {
                    win.release();
                }
                println!("destroyed");
                PostMessageAction::Remove
            }

            MessageAction::InputKey(handler) => {
                if let Some(h) = handler { h.borrow_mut().on_key(&mut self.as_window()); }
                PostMessageAction::None
            }

            MessageAction::None => {
                PostMessageAction::None
            }
        }
    }

    pub fn start_render(&self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.start_render()
        } else {
            println!("window is gone" );
            Err(ContextError::ContextLost)
        }
    }

    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.process_queue(queue)
        } else {
            println!("window is gone" );
            Err(ContextError::ContextLost)
        }
    }

    pub fn end_render(&self) -> Result<(), ContextError> {
        if let Some(ref mut win) = *self.wrapped.borrow_mut() {
            win.end_render()
        } else {
            Err(ContextError::ContextLost)
        }
    }
}

pub type WindowImpl = GLWindowWrapper;

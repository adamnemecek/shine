use render::*;

pub trait SurfaceHandler: 'static {
    fn on_ready(&mut self, window: &Window);
    fn on_lost(&mut self, window: &Window);
}

pub trait InputHandler: 'static {
    fn on_key(&mut self, window: &Window);
}

pub struct Window {
    d: WindowImpl
}

impl Window {
    pub fn new<T: Into<String>>(engine: &Engine, width: u32, height: u32, title: T) -> Result<Window, ContextError> {
        let win = try!(WindowImpl::new(&engine.platform(), width, height, title));
        Ok(Window { d: win })
    }

    pub fn new_from_impl(window_impl: WindowImpl) -> Window {
        Window { d: window_impl }
    }

    pub fn platform(&self) -> &WindowImpl {
        &self.d
    }

    pub fn platform_mut(&mut self) -> &mut WindowImpl {
        &mut self.d
    }

    pub fn is_closed(&self) -> bool {
        self.d.is_closed()
    }

    pub fn is_open(&self) -> bool {
        !self.d.is_closed()
    }

    pub fn close(&self) {
        self.d.close()
    }

    pub fn set_title(&self, title: &str) -> Result<(), ContextError> {
        self.d.set_title(title)
    }

    pub fn set_surface_handler<S: SurfaceHandler>(&self, handler: S) -> Result<(), ContextError> {
        self.d.set_surface_handler(handler)
    }

    pub fn set_input_handler<S: InputHandler>(&self, handler: S) -> Result<(), ContextError> {
        self.d.set_input_handler(handler)
    }

    pub fn start_render(&self) -> Result<(), ContextError> {
        self.d.start_render()
    }

    //fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
    //		self.d.process_queue(queue.d)
    //}

    pub fn end_render(&self) -> Result<(), ContextError> {
        self.d.end_render()
    }

    /*fn process_single_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
    try!(self.start_render());
    try!(self.process_queue(queue));
    try!(self.end_render());
    Ok(())
}*/

}

use render::*;

pub trait SurfaceHandler: 'static {
    fn on_ready(&mut self, window: &Window);
    fn on_lost(&mut self, window: &Window);
}

pub trait InputHandler: 'static {
    fn on_key(&mut self, window: &Window);
}

pub struct Window {
    pub platform: WindowImpl
}

impl Window {
    pub fn new<T: Into<String>>(engine: &Engine, width: u32, height: u32, title: T) -> Result<Window, ContextError> {
        let win = try!(WindowImpl::new(&engine, width, height, title));
        Ok(Window { platform: win })
    }

    pub fn new_platform(platform: WindowImpl) -> Window {
        Window { platform: platform }
    }

    pub fn is_closed(&self) -> bool {
        self.platform.is_closed()
    }

    pub fn is_open(&self) -> bool {
        !self.platform.is_closed()
    }

    pub fn close(&self) {
        self.platform.close()
    }

    pub fn set_title(&self, title: &str) -> Result<(), ContextError> {
        self.platform.set_title(title)
    }

    pub fn set_surface_handler<S: SurfaceHandler>(&self, handler: S) -> Result<(), ContextError> {
        self.platform.set_surface_handler(handler)
    }

    pub fn set_input_handler<S: InputHandler>(&self, handler: S) -> Result<(), ContextError> {
        self.platform.set_input_handler(handler)
    }

    pub fn start_render(&self) -> Result<(), ContextError> {
        self.platform.start_render()
    }

    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        self.platform.process_queue(queue)
    }

    pub fn end_render(&self) -> Result<(), ContextError> {
        self.platform.end_render()
    }

    pub fn process_single_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        try!(self.start_render());
        try!(self.process_queue(queue));
        try!(self.end_render());
        Ok(())
    }
}

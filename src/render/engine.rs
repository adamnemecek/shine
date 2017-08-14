use std::time::Duration;
use render::*;

pub struct Engine {
    d: EngineImpl
}

impl Engine {
    pub fn new() -> Result<Engine, ContextError> {
        let e = try!(EngineImpl::new());
        Ok(Engine { d: e })
    }

    pub fn platform(&self) -> &EngineImpl {
        &self.d
    }

    pub fn platform_mut(&mut self) -> &mut EngineImpl {
        &mut self.d
    }

    pub fn handle_message(&self, timeout: Option<Duration>) -> bool {
        self.d.handle_message(timeout)
    }

    pub fn request_quit(&mut self) {
        self.d.request_quit()
    }
}

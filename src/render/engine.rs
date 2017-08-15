use std::time::Duration;
use render::*;

pub struct Engine {
    pub platform: EngineImpl
}

impl Engine {
    pub fn new() -> Result<Engine, ContextError> {
        let e = try!(EngineImpl::new());
        Ok(Engine { platform: e })
    }

    pub fn handle_message(&self, timeout: Option<Duration>) -> bool {
        self.platform.handle_message(timeout)
    }

    pub fn request_quit(&mut self) {
        self.platform.request_quit()
    }
}

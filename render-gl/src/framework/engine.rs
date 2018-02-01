use core::*;
use framework::*;
use resources::*;


/// Engine implementation for opengl
pub struct PlatformEngine {
    platform: Box<GLEngine>
}

impl PlatformEngine {
    /// Creates a new engine.
    pub fn new() -> Result<PlatformEngine, Error> {
        let platform = try!(GLEngine::new());
        Ok(PlatformEngine { platform: platform })
    }

    pub fn platform(&self) -> &GLEngine {
        self.platform.as_ref()
    }
}

impl Engine for PlatformEngine {
    type CommandQueue/*<'a>*/ = GLCommandQueue/*<'a>*/;
    type Backend = GLBackend;

    fn quit(&self) {
        self.platform.quit();
    }

    fn dispatch_event(&self, timeout: DispatchTimeout) -> bool {
        self.platform.dispatch_event(timeout)
    }
}

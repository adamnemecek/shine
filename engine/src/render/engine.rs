#![deny(missing_docs)]

use std::time::Duration;
use render::*;

/// Enum to define the timeout policy for dispatch event
#[derive(Debug, Copy, Clone)]
pub enum DispatchTimeout {
    /// Blocking, wait for os event
    Infinite,
    /// Non-blocking, return immediately independent of event availability
    Immediate,
    /// Blocking with timeout, Wait for an event or for at most the given time
    Time(Duration),
}

/// Trait for engine abstraction.
///
/// The engine is responsible for the event loop and event dispatching.
pub trait Engine {
    /// Returns a reference to the platform specific implementation detail
    fn platform(&self) -> &EngineImpl;

    /// Returns a mutable reference to the platform specific implementation detail
    fn platform_mut(&mut self) -> &mut EngineImpl;

    /// Initiates the shutdown process.
    ///
    /// Engine is not shut down immediately, as some OS messages requires multiple cycle
    /// in the message loop. Engine has completed the shut down process once dispatch_event
    /// returns false
    fn quit(&self) {
        self.platform().quit();
    }

    /// Wait for an event to be available or for the specified timeout.
    ///
    /// Window events are delegated to the windows and shall be handled through the window.
    /// todo: If no handle event is called in a message cycle for a window, the unprocessed messages
    /// are discarded.
    /// # Return
    ///  Returns true if application is terminating, false otherwise
    fn dispatch_event(&self, timeout: DispatchTimeout) -> bool {
        self.platform().dispatch_event(timeout)
    }
}


/// Engine implementation.
///
/// The engine is responsible for the event loop and event dispatching.
pub struct PlatformEngine {
    platform: Box<EngineImpl>
}

impl PlatformEngine {
    /// Creates a new engine.
    pub fn new() -> Result<PlatformEngine, Error> {
        let platform = try!(EngineImpl::new());
        Ok(PlatformEngine { platform: platform })
    }
}

impl Engine for PlatformEngine {
    fn platform(&self) -> &EngineImpl {
        self.platform.as_ref()
    }

    fn platform_mut(&mut self) -> &mut EngineImpl {
        self.platform.as_mut()
    }
}

#![deny(missing_docs)]

use std::time::Duration;
use render::*;

/// Enum to store the error occurred during a window creation.
#[derive(Debug, Clone)]
pub enum EngineError {
    /// Engine could not be initialized error.
    /// The exact (OS) error message is also provided in the argument
    InitializeError(String),
}

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

/// Structure to store the engine abstraction.
///
/// The engine is responsible for the event loop and event dispatching.
pub struct Engine {
    platform: EngineImpl
}

impl Engine {
    /// Creates a new engine.
    pub fn new() -> Result<Engine, EngineError> {
        let platform = try!(EngineImpl::new());
        Ok(Engine { platform: platform })
    }

    /// Returns a reference to the platform specific implementation detail
    pub fn platform(&self) -> &EngineImpl {
        &self.platform
    }

    /// Returns a mutable reference to the platform specific implementation detail
    pub fn platform_mut(&mut self) -> &mut EngineImpl {
        &mut self.platform
    }

    /// Initiates the shutdown process.
    ///
    /// Engine is not shut down immediately, as some OS messages requires multiple cycle
    /// in the message loop. Engine has completed the shut down process once dispatch_event
    /// returns false
    pub fn quit(&mut self) {
        self.platform.quit();
    }

    /// Wait for an event to be available or for the specified timeout.
    ///
    /// Window events are delegated to the windows and shall be handled through the window.
    /// todo: If no handle event is called in a message cycle for a window, the unprocessed messages
    /// are discarded.
    /// # Return
    ///  Returns true if application is terminating, false otherwise
    pub fn dispatch_event(&mut self, timeout: DispatchTimeout) -> bool {
        self.platform.dispatch_event(timeout)
    }
}

#![deny(missing_docs)]

use std::time::Duration;
use resources::*;

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
pub trait Engine: 'static {
    /// Backend alias types used as alias.
    type FrameCompose: 'static;

    /// Trait defining the render backend
    type Backend: Backend<FrameCompose=Self::FrameCompose>;

    /// Initiates the shutdown process.
    ///
    /// Engine is not shut down immediately, as some OS messages requires multiple cycle
    /// in the message loop. Engine has completed the shut down process once dispatch_event
    /// returns false
    fn quit(&self);

    /// Wait for an event to be available or for the specified timeout.
    ///
    /// Window events are delegated to the windows and shall be handled through the window.
    /// todo: If no handle event is called in a message cycle for a window, the unprocessed messages
    /// are discarded.
    /// # Return
    ///  Returns true if application is terminating, false otherwise
    fn dispatch_event(&self, timeout: DispatchTimeout) -> bool;
}
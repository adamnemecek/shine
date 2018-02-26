use std::sync::Arc;
use std::sync::Barrier;
use framework::*;
use types::*;

#[derive(Clone, Debug)]
pub enum WindowCommand {
    /// Surface ready event. Called after the surface is ready for use. Hardware resource are
    /// ready to be claimed.
    SurfaceReady,

    /// Surface lost event. Called before the surface is released to clean up
    /// hardware resources.
    SurfaceLost,

    /// Indicates a Surface reconfigure/change event
    SurfaceChanged,

    /// Window closed event. Final event, by this time all resources shall be released.
    Closed,

    /// Indicates an update cycle
    Tick,

    /// Resize event with the sizes for window and draw areas.
    Resize(Size, Size),

    /// Move event with the new coordinates for the top left corners.
    Move(Position),

    /// Keyboard press event.
    KeyboardDown(ScanCode, Option<VirtualKeyCode>),

    /// Keyboard release event.
    KeyboardUp(ScanCode, Option<VirtualKeyCode>),
}


#[derive(Clone)]
pub enum WindowCmd {
    /// Asynchronous WindowsCommand command.
    Async(WindowCommand),

    /// Synchronous WindowCommand. The OS message loop is blocked while
    /// the handler has not finished processing this (and any previous) messages.
    Sync(WindowCommand, Arc<Barrier>),

    /// Request to close the window.
    RequestClose,
}

use std::sync::Arc;
use std::sync::Barrier;
use framework::*;
use types::*;

#[derive(Clone)]
pub enum WindowCommand {
    SurfaceReady,
    SurfaceLost,
    SurfaceChanged,

    /// Indicates an update cycle
    Tick,

    Resize(Size),
    Move(Position),

    KeyboardDown(ScanCode, Option<VirtualKeyCode>),
    KeyboardUp(ScanCode, Option<VirtualKeyCode>),
}


#[derive(Clone)]
pub enum WindowCmd {
    Async(WindowCommand),
    Sync(WindowCommand, Arc<Barrier>),
    RequestClose,
}

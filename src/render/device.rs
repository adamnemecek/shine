use std::time::Duration;
use super::{Window, CommandQueue};

#[derive(Debug)]
pub enum EngineFeatures {
    Robustness,
}

#[derive(Debug)]
pub enum ContextError {
    OsError(String),
    VersionNotSupported,
    FeatureNotSupported(EngineFeatures),
    NoAvailableFormat,
    IoError(::std::io::Error),
    ContextLost,
    Unknown,
}

pub trait ISurfaceHandler: 'static {
    fn on_ready(&mut self);
    fn on_lost(&mut self);
}

pub trait IWindow {
    fn close(&self);

    fn is_closed(&self) -> bool;
    fn is_open(&self) -> bool {
        !self.is_closed()
    }

    fn set_title(&self, title: &str) -> Result<(), ContextError>;
    //fn set_fullscreen();
    //fn set_size(width:u32, height: u32);
    //fn set_fullscreen();

    fn set_surface_handler<H: ISurfaceHandler>(&self, handler: H) -> Result<(), ContextError>;

    fn start_render(&self) -> Result<(), ContextError>;
    fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError>;
    fn end_render(&self) -> Result<(), ContextError>;
}

pub trait IEngine {
    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Window, ContextError>;

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool;
    fn request_quit(&mut self);
}

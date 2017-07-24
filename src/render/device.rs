use std::time::Duration;
use super::Window;

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
    fn on_ready<'a>(&mut self, win: &'a mut Window);
    fn on_lost<'a>(&mut self, win: &'a mut Window);
}

pub trait IWindow {
    fn close(&mut self);

    fn is_closed(&self) -> bool;
    fn is_open(&self) -> bool {
        !self.is_closed()
    }

    fn set_title(&mut self, title: &str) -> Result<(), ContextError>;
    //fn set_fullscreen();
    //fn set_size(width:u32, height: u32);
    //fn set_fullscreen();

    fn set_surface_handler<H: ISurfaceHandler>(&mut self, handler: H) -> Result<(), ContextError>;
    fn render_start(&mut self) -> Result<(), ContextError>;
    fn render_end(&mut self) -> Result<(), ContextError>;
}

pub trait IEngine {
    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Window, ContextError>;

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool;
    fn request_quit(&mut self);
}

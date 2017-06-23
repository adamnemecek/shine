use std::time::Duration;

#[derive(Debug)]
pub enum EngineFeatures {
    Robustness,
}

#[derive(Debug)]
pub enum EngineError {
    OsError(String),
    VersionNotSupported,
    FeatureNotSupported(EngineFeatures),
    NoAvailableFormat,
    WindowCreation(WindowError),
    Unknown,
}

#[derive(Debug)]
pub enum WindowError {
    IoError(::std::io::Error),
    ContextLost,
    Unknown,
}

pub trait RenderWindow {
    fn close(&mut self);
    fn is_closed(&self) -> bool;

    fn set_title(&self, title: &str) -> Result<(), WindowError>;

    fn handle_message(&mut self, timeout: Option<Duration>);
    fn render_start(&self) -> Result<(), WindowError>;
    fn render_end(&self) -> Result<(), WindowError>;
}

pub trait RenderEngine: Drop + Sized {
    type Window: RenderWindow;
    fn new() -> Result<Self, ()>;

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Self::Window, EngineError>;
}

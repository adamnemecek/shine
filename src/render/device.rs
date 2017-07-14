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
    //Unknown,
}

pub trait IWindow {
    fn is_closed(&self) -> bool;
    fn close(&mut self);

    fn set_title(&mut self, title: &str) -> Result<(), WindowError>;

    fn handle_message(&mut self, timeout: Option<Duration>) -> bool;
    fn render_start(&mut self) -> Result<(), WindowError>;
    fn render_end(&mut self) -> Result<(), WindowError>;
}

pub trait IEngine {
    type Window: IWindow;

    fn create_window<T: Into<String>>(&mut self, width: u32, height: u32, title: T) -> Result<Self::Window, EngineError>;
    fn close_all_windows(&mut self);
}

mod device;

pub use self::device::{IWindow, WindowError, SurfaceHandler};
pub use self::device::{IEngine, EngineFeatures, EngineError};

pub mod gl;
pub use self::gl::render::*;

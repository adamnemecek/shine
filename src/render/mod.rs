mod device;

pub use self::device::{IEngine, IWindow};
pub use self::device::{EngineFeatures, EngineError, WindowError};

pub mod gl;

#[allow(dead_code)]
pub type Window = self::gl::GLWindow;
#[allow(dead_code)]
pub type Engine = self::gl::GLEngine;

pub use self::gl::device::create_engine;

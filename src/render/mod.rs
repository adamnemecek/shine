pub mod device;

pub use self::device::{RenderEngine, RenderWindow};
pub use self::device::{EngineFeatures, EngineError, WindowError};


pub mod gl;

pub type Engine = self::gl::GLEngine;
pub type Window = self::gl::GLWindow;

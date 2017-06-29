mod device;

use std::cell::RefCell;
use std::rc::Rc;

pub use self::device::{IEngine, IWindow};
pub use self::device::{EngineFeatures, EngineError, WindowError};

pub mod gl;

pub type Window = self::gl::GLWindow;
pub type Engine = self::gl::GLEngine;
pub use self::gl::device::create_engine;

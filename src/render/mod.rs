mod device;

use std::cell::RefCell;
use std::rc::Rc;

pub use self::device::{RenderEngine, RenderWindow};
pub use self::device::{EngineFeatures, EngineError, WindowError};

pub mod gl;

pub type WindowHandle = Rc<RefCell<self::gl::GLWindow>>;
pub use self::gl::device::create_engine;

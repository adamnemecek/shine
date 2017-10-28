#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate gdi32;
#[cfg(target_os = "windows")]
extern crate user32;

#[macro_use]
mod types;
mod errors;
mod keycodes;
mod engine;
mod windowsettings;
mod window;
mod shaderprogram;
#[macro_use]
mod vertexbuffer;
mod indexbuffer;
mod texture2d;
mod renderpass;
mod renderpassconfig;
mod rendermanager;
mod commandqueue;

/// Maximum number of vertex attributes stored in a buffer.
pub const MAX_VERTEX_ATTRIBUTE_COUNT: usize = 16;


mod opengl;

pub use self::opengl::*;

pub use self::types::*;
pub use self::errors::*;
pub use self::keycodes::*;
pub use self::engine::*;
pub use self::windowsettings::*;
pub use self::window::*;
pub use self::shaderprogram::*;
pub use self::vertexbuffer::*;
pub use self::indexbuffer::*;
pub use self::texture2d::*;
pub use self::renderpass::*;
pub use self::renderpassconfig::*;
pub use self::rendermanager::*;
pub use self::commandqueue::*;



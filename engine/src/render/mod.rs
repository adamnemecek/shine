
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate shell32;
#[cfg(target_os = "windows")]
extern crate gdi32;
#[cfg(target_os = "windows")]
extern crate user32;
#[cfg(target_os = "windows")]
extern crate dwmapi;

#[macro_use]
mod types;
mod keycodes;
mod engine;
mod windowsettings;
mod window;
mod commandqueue;
mod shaderprogram;
#[macro_use]
mod vertexbuffer;

mod opengl;
pub use self::opengl::*;

pub use self::types::*;
pub use self::keycodes::*;
pub use self::engine::*;
pub use self::windowsettings::*;
pub use self::window::*;
pub use self::commandqueue::*;
pub use self::shaderprogram::*;
pub use self::vertexbuffer::*;



#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate gdi32;
#[cfg(target_os = "windows")]
extern crate user32;

#[macro_reexport(VertexDeclaration, ShaderDeclaration)]
extern crate dragorust_render_derive;

extern crate arrayvec;

#[macro_use]
mod types;
mod error;
mod keycodes;
mod engine;
mod windowsettings;
mod window;
mod shaderprogram;
#[macro_use]
mod vertexbuffer;
mod vertexsource;
mod indexbuffer;
mod indexsource;
mod texture2d;
mod texture2dsource;
mod renderpass;
mod renderpassconfig;
mod rendermanager;
mod commandqueue;

mod ops;

pub use types::*;
pub use error::*;
pub use keycodes::*;
pub use engine::*;
pub use windowsettings::*;
pub use window::*;
pub use shaderprogram::*;
pub use vertexbuffer::*;
pub use vertexsource::*;
pub use indexbuffer::*;
pub use indexsource::*;
pub use texture2d::*;
pub use texture2dsource::*;
pub use renderpass::*;
pub use renderpassconfig::*;
pub use rendermanager::*;
pub use commandqueue::*;

pub mod backend {
	#[path = "../opengl/mod.rs"]
	mod opengl;
	pub use super::*;
	pub use self::opengl::*;
}

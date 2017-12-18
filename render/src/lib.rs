#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]
#![feature(crate_visibility_modifier)]

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

extern crate dragorust_store as store;

extern crate arrayvec;

#[macro_use] mod common;
pub use common::*;

mod types;
mod error;
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
mod resources;
mod rendermanager;

mod ops;

pub use types::*;
pub use error::*;
pub use keycodes::*;
pub use engine::*;
pub use windowsettings::*;
pub use window::*;
pub use shaderprogram::*;
pub use vertexbuffer::*;
pub use indexbuffer::*;
pub use texture2d::*;
pub use renderpass::*;
pub use renderpassconfig::*;
pub use resources::*;
pub use rendermanager::*;


pub mod backend {
    #[path = "../opengl/mod.rs"]
    mod opengl;

    pub use super::*;
    pub use self::opengl::*;
}

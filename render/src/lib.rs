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

#[macro_use]
mod common;
mod engine;

pub use common::*;
pub use engine::*;

pub mod opengl;

pub mod backend {
    pub use opengl::PlatformResourceManager;
    pub use opengl::PlatformEngine;
    pub use opengl::PlatformWindow;
    pub use opengl::PlatformWindowSettings;
}

pub type RenderManager<K: PassId> = common::RenderManager<K, backend::PlatformResourceManager>;

pub use backend::PlatformEngine;
pub use backend::PlatformWindow;
pub use backend::PlatformWindowSettings;

/*

mod shaderprogram;
#[macro_use]
mod vertexbuffer;
mod indexbuffer;
mod texture2d;
mod renderpass;
mod renderpassconfig;
mod resources;
mod rendermanager;
*/

/*pub use types::*;
pub use shaderprogram::*;
pub use vertexbuffer::*;
pub use indexbuffer::*;
pub use texture2d::*;
pub use renderpass::*;
pub use renderpassconfig::*;
pub use resources::*;
pub use rendermanager::*;*/

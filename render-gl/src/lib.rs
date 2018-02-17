#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]
#![feature(crate_visibility_modifier)]
//#![feature(generic_associated_types)]

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate gdi32;
#[cfg(target_os = "windows")]
extern crate user32;

extern crate arrayvec;

extern crate shine_render_core as core;
extern crate shine_store as store;
//#[macro_reexport(ShaderDeclaration)]
//extern crate shine_render_gl_derive;

#[allow(unused_mut)]
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

/// Define engine limitations
pub mod libconfig {
    /// Maximum number of attributes that can be stored for a vertex.
    pub const MAX_VERTEX_ATTRIBUTE_COUNT: usize = 16;
    /// Maximum number of attributes that can be bound (used) at once
    pub const MAX_USED_ATTRIBUTE_COUNT: usize = 16;
    /// Maximum number of uniforms that can be bound (used) at once
    pub const MAX_USED_UNIFORM_COUNT: usize = 16;
    /// Maximum number of shader parameters including attributes and uniforms
    pub const MAX_USED_PARAMETER_COUNT: usize = MAX_USED_ATTRIBUTE_COUNT + MAX_USED_UNIFORM_COUNT;
    /// Maximum number of texture units
    pub const MAX_USED_TEXTURE_COUNT: usize = 16;
}

pub mod lowlevel;
pub mod framework;
pub mod resources;

pub use core::*;
pub use framework::*;
pub use resources::*;
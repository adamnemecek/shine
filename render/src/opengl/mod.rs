//#![cfg(target_os = "null")]
pub extern crate gl;


mod engine;
mod context;
mod window;
mod lowlevel;
mod commandqueue;
mod shaderprogram;
mod vertexbuffer;
mod indexbuffer;
mod texture;
mod renderpass;

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

pub use self::window::WindowImpl;
pub use self::engine::EngineImpl;
pub use self::lowlevel::*;
pub use self::commandqueue::*;
pub use self::shaderprogram::*;
pub use self::vertexbuffer::*;
pub use self::indexbuffer::*;
pub use self::texture::*;
pub use self::renderpass::*;

#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]
#![feature(crate_visibility_modifier)]

#[macro_reexport(VertexDeclaration, GLShaderDeclaration)]
extern crate dragorust_render_derive;

#[macro_use]
mod types;
mod error;

mod framework;
mod resources;
mod manager;

pub use self::types::*;
pub use self::error::*;
pub use self::framework::*;
pub use self::resources::*;
pub use self::manager::*;
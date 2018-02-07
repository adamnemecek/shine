#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]
#![feature(crate_visibility_modifier)]

#[macro_reexport(VertexDeclaration, GLShaderDeclaration)]
extern crate shine_render_derive;

/// Helper macro to hide unsafe ffi API calls to distinct rust-unsafe and ffi-unsafe codes
#[macro_export]
macro_rules! ffi {
    //( $cmd:ident($( $arg:expr ),*) ) => { unsafe { /*println!("{}", stringify!(gl::$cmd($($arg,)*)));*/ $cmd($($arg,)*) } };
    ( $cmd:stmt ) => { unsafe { $cmd } };
}

#[macro_use]
mod types;
mod error;

mod framework;
mod resources;

pub use self::types::*;
pub use self::error::*;
pub use self::framework::*;
pub use self::resources::*;

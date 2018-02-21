#![feature(macro_reexport)]

extern crate shine_render_core;
pub use shine_render_core::*;

#[macro_reexport(VertexDeclaration, ShaderDeclaration)]
extern crate shine_render_gl_derive;
extern crate shine_render_gl;
mod backend_impl {
    pub use super::shine_render_gl::PlatformEngine;
    pub use super::shine_render_gl::PlatformWindowSettings;
    pub use super::shine_render_gl::PlatformWindowBuilder;

    pub use super::shine_render_gl::VertexBufferHandle;
    pub use super::shine_render_gl::IndexBufferHandle;
    pub use super::shine_render_gl::Texture2DHandle;
    pub use super::shine_render_gl::ShaderProgramHandle;

    pub use super::shine_render_gl::VertexDeclaration;
    pub use super::shine_render_gl::ShaderDeclaration;
}


pub use backend_impl::*;

pub type PlatformBackend = <PlatformEngine as Engine>::Backend;
pub type PlatformCommandQueue = <PlatformEngine as Engine>::CommandQueue;

pub trait PlatformView: View<PlatformEngine> {}

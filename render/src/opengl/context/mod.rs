
pub mod wgl;
pub mod egl;

#[cfg(target_os = "windows")]
#[path = "../platform/windows/context.rs"]
mod context;

pub use self::context::Context;

pub mod wgl;
pub mod egl;

mod engine;
mod window;
mod windowsettings;

#[cfg(target_os = "windows")]
#[path = "platform/windows/engine.rs"]
mod platform_engine;

#[cfg(target_os = "windows")]
#[path = "platform/windows/window.rs"]
mod platform_window;

#[cfg(target_os = "windows")]
#[path = "platform/windows/context.rs"]
mod platform_context;

pub use self::platform_engine::GLEngine;
pub use self::platform_window::GLWindow;
pub use self::platform_window::win_messages;
pub use self::platform_context::GLContext;

pub use self::windowsettings::*;
pub use self::engine::*;
pub use self::window::*;

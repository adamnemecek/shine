#[cfg(target_os = "windows")]
#[path="platform/windows/window.rs"]
pub mod window;



pub use self::window::WindowImpl;



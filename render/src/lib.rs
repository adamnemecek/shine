#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as gfx_backend;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as gfx_backend;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as gfx_backend;

extern crate gfx_hal;
extern crate log;
extern crate winit;

mod stub;

pub use stub::stub;

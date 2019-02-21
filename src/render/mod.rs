use rendy::resource::buffer::Buffer as RendyBuffer;

#[cfg(feature = "render-dx12")]
pub type Backend = rendy::dx12::Backend;

#[cfg(feature = "render-metal")]
pub type Backend = rendy::metal::Backend;

#[cfg(feature = "render-vulkan")]
pub type Backend = rendy::vulkan::Backend;

pub type Buffer = RendyBuffer<Backend>;
pub type DescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;
pub type DescriptorSet = <Backend as gfx_hal::Backend>::DescriptorSet;
pub type DescriptorSetLayout = <Backend as gfx_hal::Backend>::DescriptorSetLayout;

pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type ShaderModule = <Backend as gfx_hal::Backend>::ShaderModule;

mod driverresource;
mod frameparameters;
mod graph;

pub use self::driverresource::*;
pub use self::frameparameters::*;
pub use self::graph::*;

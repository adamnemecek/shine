#[cfg(feature = "render-dx12")]
pub type Backend = rendy::dx12::Backend;

#[cfg(feature = "render-metal")]
pub type Backend = rendy::metal::Backend;

#[cfg(feature = "render-vulkan")]
pub type Backend = rendy::vulkan::Backend;

pub type Buffer = rendy::resource::buffer::Buffer<Backend>;
pub type Factory = rendy::factory::Factory<Backend>;
pub type GraphContext = rendy::graph::GraphContext<Backend>;

//pub type DescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;
pub type DescriptorSet = rendy::resource::set::DescriptorSet<Backend>;
pub type DescriptorSetLayout = rendy::resource::set::DescriptorSetLayout<Backend>;
pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type ShaderModule = <Backend as gfx_hal::Backend>::ShaderModule;

pub type Mesh = rendy::mesh::Mesh<Backend>;
pub type MeshBuilder<'a> = rendy::mesh::MeshBuilder<'a>;

pub trait IntoMesh {
    fn into_mesh(&self) -> MeshBuilder<'static>;
}

mod driverresource;
mod frameinfo;
mod frameparameters;
mod graph;

pub use self::driverresource::*;
pub use self::frameinfo::*;
pub use self::frameparameters::*;
pub use self::graph::*;

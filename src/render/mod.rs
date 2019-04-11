#[cfg(feature = "render-dx12")]
pub type Backend = rendy::dx12::Backend;

#[cfg(feature = "render-metal")]
pub type Backend = rendy::metal::Backend;

#[cfg(feature = "render-vulkan")]
pub type Backend = rendy::vulkan::Backend;

pub type Buffer = rendy::resource::Buffer<Backend>;
pub type Factory = rendy::factory::Factory<Backend>;
pub type GraphContext = rendy::graph::GraphContext<Backend>;

//pub type DescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;
pub type DescriptorSet = rendy::resource::DescriptorSet<Backend>;
pub type DescriptorSetLayout = rendy::resource::DescriptorSetLayout<Backend>;
pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type ShaderModule = <Backend as gfx_hal::Backend>::ShaderModule;

pub type Mesh = rendy::mesh::Mesh<Backend>;
pub type MeshBuilder<'a> = rendy::mesh::MeshBuilder<'a>;

pub trait IntoMesh {
    fn into_mesh(&self) -> MeshBuilder<'static>;
}

mod components;
mod driverresource;
mod graph;

pub use self::components::*;
pub use self::driverresource::*;
pub use self::framelimiter::*;
pub use self::frametimer::*;
pub use self::graph::*;

use shine_ecs::world::{EntityWorld, ResourceWorld, World};

pub fn prepare_world(world: &mut World) {
    world.register_resource::<FrameParameters>();
    world.register_entity_component::<SimpleMeshData>();
    world.register_entity_component::<SimpleMesh>();
}

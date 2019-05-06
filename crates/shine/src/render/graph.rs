use crate::render::{Backend, Buffer, DescriptorSet, DescriptorSetLayout, Factory, GraphContext, PipelineLayout, ShaderSet};
use crate::render::{FrameInfo, FrameParameters};
use gfx_hal::device::Device;
use lazy_static::lazy_static;
use rendy::command::{Families, QueueId, RenderPassEncoder};
use rendy::graph::present::PresentNode;
use rendy::graph::render::{
    Layout, PrepareResult, RenderGroupBuilder, SetLayout, SimpleGraphicsPipeline, SimpleGraphicsPipelineDesc,
};
use rendy::graph::{GraphBuilder, NodeBuffer, NodeImage};
use rendy::memory::MemoryUsageValue;
use rendy::mesh::{AsVertex, PosColor};
use rendy::resource::{BufferInfo, Escape, Handle};
use rendy::shader::{ShaderKind, SourceLanguage, StaticShaderInfo};
use rendy::wsi::Surface;
use shine_ecs::world::{ResourceWorld, World};
use shine_shard::camera::RenderCamera;

pub type Graph = rendy::graph::Graph<Backend, World>;

lazy_static! {
    static ref SHADERS: rendy::shader::ShaderSetBuilder = rendy::shader::ShaderSetBuilder::default()
        .with_vertex(&StaticShaderInfo::new("assets/shaders/tri.vert", ShaderKind::Vertex, SourceLanguage::GLSL, "main",)).unwrap()
        .with_fragment(&StaticShaderInfo::new("assets/shaders/tri.frag", ShaderKind::Fragment, SourceLanguage::GLSL, "main",)).unwrap();
}
/*
struct Context {
    frame_params: * mut FrameParameters,
}

impl Context {

}*/

#[derive(Debug, Default)]
struct TriangleRenderPipelineDesc;

impl SimpleGraphicsPipelineDesc<Backend, World> for TriangleRenderPipelineDesc {
    type Pipeline = TriangleRenderPipeline;

    fn layout(&self) -> Layout {
        Layout {
            sets: vec![SetLayout {
                bindings: vec![gfx_hal::pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: gfx_hal::pso::DescriptorType::UniformBuffer,
                    count: 1,
                    stage_flags: gfx_hal::pso::ShaderStageFlags::GRAPHICS,
                    immutable_samplers: false,
                }],
            }],
            push_constants: Vec::new(),
        }
    }

    fn vertices(
        &self,
    ) -> Vec<(
        Vec<gfx_hal::pso::Element<gfx_hal::format::Format>>,
        gfx_hal::pso::ElemStride,
        gfx_hal::pso::InstanceRate,
    )> {
        vec![PosColor::vertex().gfx_vertex_input_desc(0)]
    }

    fn load_shader_set<'a>(&self, factory: &mut Factory, _world: &World) -> ShaderSet {
        SHADERS.build(factory).unwrap()
    }

    fn build<'a>(
        self,
        _context: &GraphContext,
        factory: &mut Factory,
        _queue: QueueId,
        world: &World,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout>],
    ) -> Result<TriangleRenderPipeline, failure::Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert_eq!(set_layouts.len(), 1);

        let frame_parameters = world.resource::<FrameParameters>();
        let frames = frame_parameters.frame_count();

        let mut descriptor_sets = Vec::new();
        for index in 0..frames {
            let projection_range = frame_parameters.projection_args_range(index);
            let buffer = frame_parameters.buffer();

            unsafe {
                let set = factory.create_descriptor_set(set_layouts[0].clone()).unwrap();
                factory.write_descriptor_sets(Some(gfx_hal::pso::DescriptorSetWrite {
                    set: set.raw(),
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(gfx_hal::pso::Descriptor::Buffer(buffer.raw(), projection_range)),
                }));
                descriptor_sets.push(set);
            }
        }

        Ok(TriangleRenderPipeline {
            vertex: None,
            descriptor_sets,
        })
    }
}

#[derive(Debug)]
struct TriangleRenderPipeline {
    descriptor_sets: Vec<Escape<DescriptorSet>>,
    vertex: Option<Escape<Buffer>>,
}

impl SimpleGraphicsPipeline<Backend, World> for TriangleRenderPipeline {
    type Desc = TriangleRenderPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory,
        _queue: QueueId,
        _set_layouts: &[Handle<DescriptorSetLayout>],
        index: usize,
        world: &World,
    ) -> PrepareResult {
        let frame_info = world.resource::<FrameInfo>();
        let camera = world.resource::<RenderCamera>();
        world
            .resource_mut::<FrameParameters>()
            .update(factory, index, frame_info.frame_id, &*camera);

        if self.vertex.is_none() {
            let mut vbuf = factory
                .create_buffer(
                    BufferInfo {
                        size: PosColor::vertex().stride as u64 * 3,
                        usage: gfx_hal::buffer::Usage::VERTEX,
                    },
                    MemoryUsageValue::Dynamic,
                )
                .unwrap();

            unsafe {
                // Fresh buffer.
                factory
                    .upload_visible_buffer(
                        &mut vbuf,
                        0,
                        &[
                            PosColor {
                                position: [0.0, -0.5, 0.0].into(),
                                color: [1.0, 0.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [0.5, 0.5, 0.0].into(),
                                color: [0.0, 1.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [-0.5, 0.5, 0.0].into(),
                                color: [0.0, 0.0, 1.0, 1.0].into(),
                            },
                        ],
                    )
                    .unwrap();
            }

            self.vertex = Some(vbuf);
        }

        PrepareResult::DrawReuse
    }

    fn draw(&mut self, layout: &PipelineLayout, mut encoder: RenderPassEncoder<'_, Backend>, index: usize, _world: &World) {
        encoder.bind_graphics_descriptor_sets(layout, 0, Some(self.descriptor_sets[index].raw()), std::iter::empty());

        let vbuf = self.vertex.as_ref().unwrap();
        encoder.bind_vertex_buffers(0, Some((vbuf.raw(), 0)));
        encoder.draw(0..3, 0..1);
    }

    fn dispose(self, _factory: &mut Factory, _world: &World) {}
}

pub fn init(factory: &mut Factory, families: &mut Families<Backend>, surface: Surface<Backend>, world: &World) -> Graph {
    let mut graph_builder = GraphBuilder::<Backend, World>::new();

    let color = graph_builder.create_image(
        surface.kind(),
        1,
        factory.get_surface_format(&surface),
        Some(gfx_hal::command::ClearValue::Color([1.0, 1.0, 1.0, 1.0].into())),
    );

    let depth = graph_builder.create_image(
        surface.kind(),
        1,
        gfx_hal::format::Format::D16Unorm,
        Some(gfx_hal::command::ClearValue::DepthStencil(
            gfx_hal::command::ClearDepthStencil(1.0, 0),
        )),
    );

    let color_pass_builder = TriangleRenderPipeline::builder()
        .into_subpass()
        .with_color(color)
        .with_depth_stencil(depth);
    let color_pass = graph_builder.add_node(color_pass_builder.into_pass());

    let present_pass_builder = PresentNode::builder(&factory, surface, color).with_dependency(color_pass);
    let frame_count = present_pass_builder.image_count() as usize;
    let _present_pass = graph_builder.add_node(present_pass_builder);

    {
        let mut frame_params = world.resource_mut::<FrameParameters>();
        frame_params.init(factory, frame_count);
    }

    graph_builder.build(factory, families, world).unwrap()
}

pub fn dispose(factory: &mut Factory, world: &World) {
    log::trace!("disposing world");
    world.resource_mut::<FrameParameters>().dispose(factory);
}

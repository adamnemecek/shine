use crate::render::{
    Backend, Buffer, DescriptorSet, DescriptorSetLayout, Factory, GraphContext, PipelineLayout, ShaderModule,
};
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
use rendy::shader::{Shader, ShaderKind, SourceLanguage, StaticShaderInfo};
use rendy::wsi::Surface;
use shine_ecs::{ResourceWorld, World};
use shine_math::camera::FpsCamera;

pub type Graph = rendy::graph::Graph<Backend, World>;

lazy_static! {
    static ref VERTEX: StaticShaderInfo =
        StaticShaderInfo::new("assets/shaders/tri.vert", ShaderKind::Vertex, SourceLanguage::GLSL, "main",);
    static ref FRAGMENT: StaticShaderInfo =
        StaticShaderInfo::new("assets/shaders/tri.frag", ShaderKind::Fragment, SourceLanguage::GLSL, "main",);
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
        vec![PosColor::VERTEX.gfx_vertex_input_desc(0)]
    }

    fn load_shader_set<'a>(
        &self,
        storage: &'a mut Vec<ShaderModule>,
        factory: &mut Factory,
        _world: &World,
    ) -> gfx_hal::pso::GraphicsShaderSet<'a, Backend> {
        storage.clear();

        log::trace!("Load shader module '{:#?}'", *VERTEX);
        storage.push(VERTEX.module(factory).unwrap());

        log::trace!("Load shader module '{:#?}'", *FRAGMENT);
        storage.push(FRAGMENT.module(factory).unwrap());

        gfx_hal::pso::GraphicsShaderSet {
            vertex: gfx_hal::pso::EntryPoint {
                entry: "main",
                module: &storage[0],
                specialization: gfx_hal::pso::Specialization::default(),
            },
            fragment: Some(gfx_hal::pso::EntryPoint {
                entry: "main",
                module: &storage[1],
                specialization: gfx_hal::pso::Specialization::default(),
            }),
            hull: None,
            domain: None,
            geometry: None,
        }
    }

    fn build<'a>(
        self,
        _context: &mut GraphContext,
        factory: &mut Factory,
        _queue: QueueId,
        world: &World,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[DescriptorSetLayout],
    ) -> Result<TriangleRenderPipeline, failure::Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert_eq!(set_layouts.len(), 1);

        let frame_parameters = world.get_resource::<FrameParameters>();
        let frames = frame_parameters.frame_count();

        let mut descriptor_sets = Vec::new();
        for index in 0..frames {
            let projection_range = frame_parameters.projection_args_range(index);
            let buffer = frame_parameters.buffer();

            unsafe {
                let set =  factory.create_descriptor_set(&set_layouts[0]).unwrap();
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
    descriptor_sets: Vec<DescriptorSet>,
    vertex: Option<Buffer>,
}

impl SimpleGraphicsPipeline<Backend, World> for TriangleRenderPipeline {
    type Desc = TriangleRenderPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory,
        _queue: QueueId,
        _set_layouts: &[DescriptorSetLayout],
        index: usize,
        world: &World,
    ) -> PrepareResult {
        let frame_info = world.get_resource::<FrameInfo>();
        let camera = world.get_resource::<FpsCamera>();
        let mut frame_parameters = world.get_resource_mut::<FrameParameters>();
        frame_parameters.update(factory, index, frame_info.frame_id, &*camera);

        if self.vertex.is_none() {
            let mut vbuf = factory
                .create_buffer(
                    512,
                    PosColor::VERTEX.stride as u64 * 3,
                    (gfx_hal::buffer::Usage::VERTEX, MemoryUsageValue::Dynamic),
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

pub fn init(factory: &mut Factory, families: &mut Families<Backend>, surface: Surface<Backend>, world: &mut World) -> Graph {
    let mut graph_builder = GraphBuilder::<Backend, World>::new();

    let color = graph_builder.create_image(
        surface.kind(),
        1,
        factory.get_surface_format(&surface),
        MemoryUsageValue::Data,
        Some(gfx_hal::command::ClearValue::Color([1.0, 1.0, 1.0, 1.0].into())),
    );

    let depth = graph_builder.create_image(
        surface.kind(),
        1,
        gfx_hal::format::Format::D16Unorm,
        MemoryUsageValue::Data,
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

    world.register_resource_with(FrameParameters::new(factory, frame_count));

    graph_builder.build(factory, families, world).unwrap()
}

pub fn dispose(factory: &mut Factory, world: &mut World) {
    log::trace!("disposing world");
    world.get_resource_mut::<FrameParameters>().dispose(factory);
}

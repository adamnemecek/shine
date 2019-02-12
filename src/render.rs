use lazy_static::lazy_static;
use rendy::command::RenderPassEncoder;
use rendy::factory::Factory;
use rendy::graph::present::PresentNode;
use rendy::graph::render::{PrepareResult, RenderGroupBuilder, SimpleGraphicsPipeline};
use rendy::graph::{GraphBuilder, NodeBuffer, NodeImage};
use rendy::memory::MemoryUsageValue;
use rendy::mesh::{AsVertex, PosColor};
use rendy::resource::buffer::Buffer;
use rendy::shader::{Shader, ShaderKind, SourceLanguage, StaticShaderInfo};
use rendy::wsi::Surface;
use shine_ecs::World;

#[cfg(feature = "render-dx12")]
pub type Backend = rendy::dx12::Backend;

#[cfg(feature = "render-metal")]
pub type Backend = rendy::metal::Backend;

#[cfg(feature = "render-vulkan")]
pub type Backend = rendy::vulkan::Backend;

pub type Graph = rendy::graph::Graph<Backend, World>;

lazy_static! {
    static ref VERTEX: StaticShaderInfo =
        StaticShaderInfo::new("assets/shaders/tri.vert", ShaderKind::Vertex, SourceLanguage::GLSL, "main",);
    static ref FRAGMENT: StaticShaderInfo =
        StaticShaderInfo::new("assets/shaders/tri.frag", ShaderKind::Fragment, SourceLanguage::GLSL, "main",);
}

#[derive(Debug)]
struct TriangleRenderGroup<B: gfx_hal::Backend> {
    vertex: Option<Buffer<B>>,
}

impl<B, T> SimpleGraphicsPipeline<B, T> for TriangleRenderGroup<B>
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    fn name() -> &'static str {
        "Triangle"
    }

    fn vertices() -> Vec<(
        Vec<gfx_hal::pso::Element<gfx_hal::format::Format>>,
        gfx_hal::pso::ElemStride,
        gfx_hal::pso::InstanceRate,
    )> {
        vec![PosColor::VERTEX.gfx_vertex_input_desc(0)]
    }

    fn depth() -> bool {
        false
    }

    fn load_shader_set<'a>(
        storage: &'a mut Vec<B::ShaderModule>,
        factory: &mut Factory<B>,
        _aux: &mut T,
    ) -> gfx_hal::pso::GraphicsShaderSet<'a, B> {
        storage.clear();

        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

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
        _factory: &mut Factory<B>,
        _aux: &mut T,
        buffers: Vec<NodeBuffer<'a, B>>,
        images: Vec<NodeImage<'a, B>>,
        set_layouts: &[B::DescriptorSetLayout],
    ) -> Result<Self, failure::Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert!(set_layouts.is_empty());

        Ok(TriangleRenderGroup { vertex: None })
    }

    fn prepare(
        &mut self,
        factory: &mut Factory<B>,
        _set_layouts: &[B::DescriptorSetLayout],
        _index: usize,
        _aux: &T,
    ) -> PrepareResult {
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

    fn draw(&mut self, _layout: &B::PipelineLayout, mut encoder: RenderPassEncoder<'_, B>, _index: usize, _aux: &T) {
        let vbuf = self.vertex.as_ref().unwrap();
        encoder.bind_vertex_buffers(0, Some((vbuf.raw(), 0)));
        encoder.draw(0..3, 0..1);
    }

    fn dispose(self, _factory: &mut Factory<B>, _aux: &mut T) {}
}

pub fn init(factory: &mut Factory<Backend>, surface: Surface<Backend>, world: &mut World) -> Graph {
    let mut graph_builder = GraphBuilder::<Backend, World>::new();

    let color = graph_builder.create_image(
        surface.kind(),
        1,
        factory.get_surface_format(&surface),
        MemoryUsageValue::Data,
        Some(gfx_hal::command::ClearValue::Color([1.0, 1.0, 1.0, 1.0].into())),
    );

    let pass = graph_builder.add_node(TriangleRenderGroup::builder().into_subpass().with_color(color).into_pass());

    graph_builder.add_node(PresentNode::builder(surface, color).with_dependency(pass));

    graph_builder.build(factory, world).unwrap()
}

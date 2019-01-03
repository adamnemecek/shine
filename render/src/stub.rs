use gfx_backend;
use gfx_hal::format::{Aspects, ChannelType, Format, Swizzle};
use gfx_hal::image::{Access, Extent, Layout, SubresourceRange, ViewKind};
use gfx_hal::pass::{
    Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency, SubpassDesc, SubpassRef,
};
use gfx_hal::pool::{CommandPool, CommandPoolCreateFlags};
use gfx_hal::pso::PipelineStage;
use gfx_hal::queue::{CommandQueue, QueueGroup};
use gfx_hal::window::Extent2D;
use gfx_hal::{Adapter, Backbuffer, Device, FrameSync, Graphics, Instance, Surface, SwapImageIndex, Swapchain, SwapchainConfig};
use log::{debug, info, trace};
use std::mem;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder, WindowEvent};

//todo: config
//  monitor id
//  graphics adapter

struct RenderTree<B>
where
    B: gfx_hal::Backend,
{
    color_format: Format,
    swapchain: B::Swapchain,
    extent: Extent,
    render_pass: Option<B::RenderPass>,
    frameviews: Vec<B::ImageView>,
    framebuffers: Vec<B::Framebuffer>,
}

impl<B> RenderTree<B>
where
    B: gfx_hal::Backend,
{
    fn new(physical_device: &B::PhysicalDevice, surface: &mut B::Surface, device: &B::Device, old_swapchain: Option<B::Swapchain>) -> RenderTree<B> {
        info!("Creating rendertree");

        let (caps, formats, _) = surface.compatibility(physical_device);
        debug!("caps: {:?}", caps);
        debug!("formats: {:?}", formats);

        // find a best fitting srgb format for the surface
        let color_format = formats
            .and_then(|formats| {
                formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .map(|format| *format)
            })
            .unwrap_or(Format::Rgba8Srgb);
        info!("color_format: {:?}", color_format);

        let swap_config = SwapchainConfig::from_caps(&caps, color_format, caps.extents.start);
        let extent = swap_config.extent.to_extent();
        debug!("extent: {:?}", extent);
        let (swapchain, backbuffer) = device.create_swapchain(surface, swap_config, old_swapchain).unwrap();

        let (render_pass, frameviews, framebuffers) = Self::create_render_pass(device, color_format);

        RenderTree {
            color_format,
            extent,
            swapchain,
            Some(render_pass),
            frameviews,
            framebuffers,
        }
    }

    fn create_render_pass(device: &B::Device, surface_color_format: Format) -> (B::RenderPass, Vec<B::ImageView>, Vec<B::Framebuffer>) {
        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let dependency = SubpassDependency {
            passes: SubpassRef::External..SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: Access::empty()..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
        };

        let pass = device
            .create_render_pass(&[color_attachment], &[subpass], &[dependency])
            .unwrap();

        let (frameviews, framebuffers) = match backbuffer {
            Backbuffer::Images(images) => {
                let color_range = SubresourceRange {
                    aspects: Aspects::COLOR,
                    levels: 0..1,
                    layers: 0..1,
                };

                let image_views = images
                    .iter()
                    .map(|image| {
                        device
                            .create_image_view(image, ViewKind::D2, color_format, Swizzle::NO, color_range.clone())
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                let framebuffers = image_views
                    .iter()
                    .map(|image_view| device.create_framebuffer(&pass, vec![image_view], extent).unwrap())
                    .collect();

                (image_views, framebuffers)
            }
            Backbuffer::Framebuffer(framebuffer) => (Vec::new(), vec![framebuffer]),
        };

        (pass, framebuffers, frameviews)
    }

    fn release(self, device: &B::Device) -> B::Swapchain {
        for framebuffer in self.framebuffers {
            device.destroy_framebuffer(framebuffer);
        }
        for view in self.frameviews {
            device.destroy_image_view(view);
        }
        device.destroy_render_pass(self.render_pass);        
        self.swapchain
    }

    fn swap(&mut self, semaphore: &B::Semaphore) -> Result<SwapImageIndex, ()> {
        self.swapchain
            .acquire_image(!0, FrameSync::Semaphore(semaphore))
            .map_err(|err| {
                info!("swap error: {:?}", err);
                ()
            })
    }

    fn present(
        &mut self,
        frame_index: SwapImageIndex,
        queue: &mut CommandQueue<B, Graphics>,
        semaphore: &B::Semaphore,
    ) -> Result<(), ()> {
        self.swapchain.present(queue, frame_index, vec![semaphore]).map_err(|err| {
            info!("present error: {:?}", err);
            ()
        })
    }
}

struct Pools<B>
where
    B: gfx_hal::Backend,
{
    frame_semaphore: B::Semaphore,
    present_semaphore: B::Semaphore,
    command_pool: CommandPool<B, Graphics>,
}

struct Stub<B>
where
    B: gfx_hal::Backend,
{
    instance: gfx_backend::Instance,
    adapter: Adapter<B>,
    surface: B::Surface,
    device: B::Device,

    queue_group: QueueGroup<B, Graphics>,
    pools: Option<Pools<B>>,

    rebuild_render_tree: bool,
    render_tree: Option<RenderTree<B>>,
}

impl<B> Stub<B>
where
    B: gfx_hal::Backend,
{
    fn release_render_tree(&mut self) -> Option<B::Swapchain> {
        self.device.wait_idle().unwrap();
        let pools = self.pools.as_mut().unwrap();
        pools.command_pool.reset();
        if let Some(render_tree) = self.render_tree.take() {
            Some(render_tree.release(&self.device))
        } else {
            None
        }
    }

    fn prepare(&mut self) -> Result<(), ()> {
        let old_swapchain = if self.rebuild_render_tree {
            self.rebuild_render_tree = false;
            self.release_render_tree()
        } else {
            None
        };

        if self.render_tree.is_none() {
            let render_tree = RenderTree::new(&self.adapter.physical_device, &mut self.surface, &self.device, old_swapchain);
            self.render_tree = Some(render_tree);
        }

        Ok(())
    }

    fn request_rebuild(&mut self) {
        self.rebuild_render_tree = true;
    }

    fn swap(&mut self) -> Result<SwapImageIndex, ()> {
        let pools = self.pools.as_mut().unwrap();
        if let Some(render_tree) = self.render_tree.as_mut() {
            match render_tree.swap(&pools.frame_semaphore) {
                ok @ Ok(_) => ok,
                err @ Err(_) => {
                    self.rebuild_render_tree = true;
                    err
                }
            }
        } else {
            Err(())
        }
    }

    fn present(&mut self, frame_index: SwapImageIndex) -> Result<(), ()> {
        let pools = self.pools.as_mut().unwrap();
        if let Some(render_tree) = self.render_tree.as_mut() {
            match render_tree.present(frame_index, &mut self.queue_group.queues[0], &pools.present_semaphore) {
                ok @ Ok(_) => ok,
                err @ Err(_) => {
                    self.rebuild_render_tree = true;
                    err
                }
            }
        } else {
            Err(())
        }
    }

    fn render(&mut self) -> Result<(), ()> {
        self.prepare()?;
        {        
            let pools = self.pools.as_mut().unwrap();
            pools.command_pool.reset();
        }
        let frame_index = self.swap()?;
        //todo, render
        self.present(frame_index)?;
        Ok(())
    }
}

impl<B> Drop for Stub<B>
where
    B: gfx_hal::Backend,
{
    fn drop(&mut self) {
        let swap_chain = self.release_render_tree();
        let pools = self.pools.take().unwrap();
        self.device.destroy_command_pool(pools.command_pool.into_raw());
        self.device.destroy_semaphore(pools.frame_semaphore);
        self.device.destroy_semaphore(pools.present_semaphore);
        if let Some(swap_chain) = swap_chain {
            self.device.destroy_swapchain(swap_chain);
        }
    }
}

impl Stub<gfx_backend::Backend> {
    fn new(window: &Window) -> Stub<gfx_backend::Backend> {
        let instance = gfx_backend::Instance::create("shine-render", 1);

        let adapter = {
            let mut adapters = instance.enumerate_adapters();
            debug!("adapters:");
            for (id, adapter) in adapters.iter().enumerate() {
                debug!("{}. {:?}", id, adapter.info);
            }
            adapters.remove(0)
        };
        info!("adapter: {:?}", adapter.info);

        let mut surface = instance.create_surface(window);

        let (mut device, mut queue_group) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        let frame_semaphore = device.create_semaphore().unwrap();
        let present_semaphore = device.create_semaphore().unwrap();
        let command_pool = device
            .create_command_pool_typed(&queue_group, CommandPoolCreateFlags::empty(), 16)
            .unwrap();

        Stub {
            instance,
            adapter,
            surface,
            device,
            queue_group,
            pools: Some(Pools {
                frame_semaphore,
                present_semaphore,
                command_pool,
            }),
            rebuild_render_tree: true,
            render_tree: None,
        }
    }
}

pub fn stub(title: &str) {
    let mut events_loop = EventsLoop::new();
    let monitor = events_loop.get_primary_monitor();
    info!(
        "monitor: {:?}, {:?}, hidpi:{}",
        monitor.get_name(),
        monitor.get_dimensions(),
        monitor.get_hidpi_factor()
    );

    let window = WindowBuilder::new()
        //.with_fullscreen(Some(monitor))
        .with_dimensions((640, 480).into())
        .with_title(title)
        .build(&events_loop)
        .unwrap();

    let mut stub = Stub::new(&window);

    loop {
        let mut quitting = false;

        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => quitting = true,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => quitting = true,
                    WindowEvent::Resized(_) => {
                        stub.request_rebuild();
                    }
                    _ => {}
                }
            }
        });

        if quitting {
            break;
        }

        let _ = stub.render();
    }

    mem::drop(stub);
}

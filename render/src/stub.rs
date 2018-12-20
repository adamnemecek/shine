use gfx_backend;
use gfx_hal::format::{Aspects, ChannelType, Format, Swizzle};
use gfx_hal::image::{Extent, SubresourceRange, ViewKind};
use gfx_hal::window::Extent2D;
use gfx_hal::{Adapter, Backbuffer, Device, Instance, Surface, Swapchain, SwapchainConfig};
use log::{debug, info, trace};
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};

//todo: config
//  monitor id
//  graphics adapter

struct CompositionInner<B>
where
    B: gfx_hal::Backend,
{
    color_format: Format,
    swapchain: B::Swapchain,
    extent: Extent,
    frameviews: Vec<B::ImageView>,
    framebuffers: Vec<B::Framebuffer>,
}

struct Composition<'a, B>
where
    B: gfx_hal::Backend,
{
    device: &'a B::Device,

    inner: Option<CompositionInner<B>>,
}

impl<'a, B> Composition<'a, B>
where
    B: gfx_hal::Backend,
{
    fn new<'b>(physical_device: &B::PhysicalDevice, surface: &mut B::Surface, device: &'b B::Device) -> Composition<'b, B> {
        info!("Creating composition");

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
        let (swapchain, backbuffer) = device.create_swapchain(surface, swap_config, None).unwrap();

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

                /*let framebuffers = image_views
                .iter()
                .map(|image_view| {
                    device
                        .create_framebuffer(&render_pass, vec![image_view], extent)
                        .unwrap()
                }).collect();*/

                (image_views, Vec::new() /*framebuffers*/)
            }
            Backbuffer::Framebuffer(framebuffer) => (Vec::new(), vec![framebuffer]),
        };

        Composition {
            device,
            inner: Some(CompositionInner {
                color_format,
                extent,
                swapchain,
                frameviews,
                framebuffers,
            }),
        }
    }
}
impl<'a, B> Drop for Composition<'a, B>
where
    B: gfx_hal::Backend,
{
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            // We want to wait for all queues to be idle and reset the command pool,
            // so that we know that no commands are being executed while we destroy
            // the swapchain.
            self.device.wait_idle().unwrap();
            //self.command_pool.reset();

            for framebuffer in inner.framebuffers {
                self.device.destroy_framebuffer(framebuffer);
            }

            for view in inner.frameviews {
                self.device.destroy_image_view(view);
            }

            self.device.destroy_swapchain(inner.swapchain);
        }
    }
}

pub fn stub(title: &str) {
    let command_queue_count = 1;
    let max_command_buffers = 16;

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

    const VERSION: u32 = 1;

    let instance = gfx_backend::Instance::create(title, VERSION);
    let adapter = {
        let mut adapters = instance.enumerate_adapters();
        debug!("adapters:");
        for (id, adapter) in adapters.iter().enumerate() {
            debug!("{}. {:?}", id, adapter.info);
        }
        adapters.remove(0)
    };
    info!("adapter: {:?}", adapter.info);

    let mut surface = instance.create_surface(&window);
    let physical_device = &adapter.physical_device;
    let (mut render_device, mut queue_group) = adapter
        .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
        .unwrap();

    /*let mut command_pool = device.create_command_pool_typed(
        &queue_group,
        CommandPoolCreateFlags::empty(),
        max_command_buffers,
    );*/

    let mut composition: Option<Composition<gfx_backend::Backend>> = None;

    loop {
        let mut quitting = false;
        let mut rebuild_swapchain = false;

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
                        rebuild_swapchain = true;
                    }
                    _ => {}
                }
            }
        });

        if rebuild_swapchain || quitting {
            composition = None;
        }

        if quitting {
            break;
        }

        if composition.is_none() {
            composition = Some(Composition::new(physical_device, &mut surface, &render_device));
        }
    }
}

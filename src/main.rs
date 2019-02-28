#![feature(core_intrinsics)]

use gilrs::{Button as GilButton, Event as GilEvent, Gilrs};
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::{ResourceWorld, World};
use shine_math::camera::FpsCamera;
use std::env;
use std::sync::Arc;
use winit::{Event as WinEvent, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent};

mod guestures;
mod render;

use guestures::Guestures;

#[derive(Copy, Clone, Debug, PartialEq)]
enum EventResult {
    None,
    SurfaceLost,
    Closing,
}

fn handle_events(world: &mut World, event_loop: &mut EventsLoop, gilrs: &mut Gilrs) -> EventResult {
    let mut guestures = world.get_resource_mut::<Guestures>();
    let mut is_closing = false;
    let mut is_surface_lost = false;

    // poll window events
    event_loop.poll_events(|e| match e {
        WinEvent::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => is_closing = true,
            /*WindowEvent::Resized { .. } => {
                release_graph = true;
            }*/
            WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => is_closing = true,
                    Some(VirtualKeyCode::F11) => is_surface_lost = true,
                    //Some(VirtualKeyCode::Down) => guestures.up = -1.,
                    //Some(VirtualKeyCode::Up) => guestures.up = 1.,
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    });

    // poll gil events
    if !is_closing {
        while let Some(GilEvent { id, event, time }) = gilrs.next_event() {
            use gilrs::{Axis, EventType::AxisChanged};
            match event {
                AxisChanged(Axis::LeftStickY, v, ..) => guestures.forward = v,
                AxisChanged(Axis::LeftStickX, v, ..) => guestures.side = v,
                AxisChanged(Axis::RightStickX, v, ..) => guestures.yaw = v,
                AxisChanged(Axis::RightStickY, v, ..) => guestures.pitch = v,
                _ => {},
            }
            log::trace!("{:?} New event from {}: {:?}", time, id, event);
        }
    }

    if is_closing {
        EventResult::Closing
    } else if is_surface_lost {
        EventResult::SurfaceLost
    } else {
        EventResult::None
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("shine", log::LevelFilter::Trace)
        .init();

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        log::info!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    log::trace!("current executable {:?}", env::current_exe());
    log::trace!("current path {:?}", env::current_dir());
    let mut world = World::new();
    world.register_resource_with(FpsCamera::new());
    world.register_resource_with(Guestures::new());
    world.register_resource_with(render::FrameInfo::new());

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();
    let mut event_loop = EventsLoop::new();

    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());
    let mut graph: Option<render::Graph> = None;

    loop {
        factory.maintain(&mut families);

        let event_result = handle_events(&mut world, &mut event_loop, &mut gilrs);

        if event_result == EventResult::SurfaceLost || event_result == EventResult::Closing {
            if let Some(graph) = graph.take() {
                graph.dispose(&mut factory, &mut world);
                render::dispose(&mut factory, &mut world);
            }
        }

        if event_result == EventResult::Closing {
            log::trace!("closing");
            break;
        }

        {
            let guestures = world.get_resource::<Guestures>();
            let mut frame = world.get_resource_mut::<render::FrameInfo>();
            frame.frame_id += 1;

            let mut cam = world.get_resource_mut::<FpsCamera>();

            cam.move_up(guestures.up * 0.001);
            cam.move_forward(guestures.forward * 0.01);
            cam.move_side(guestures.side * 0.001);
            cam.yaw(-guestures.yaw * 0.001);
            cam.pitch(guestures.pitch * 0.001);
            cam.roll(guestures.roll * 0.001);
        }

        if graph.is_none() {
            let surface = factory.create_surface(window.clone());
            graph = Some(render::init(&mut factory, &mut families, surface, &mut world));
        }

        if let Some(ref mut graph) = graph {
            graph.run(&mut factory, &mut families, &mut world);
        }
    }

    log::trace!("bye.");
}

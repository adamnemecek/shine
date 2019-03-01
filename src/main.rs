#![feature(core_intrinsics)]

use gilrs::Gilrs;
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::{ResourceWorld, World};
use shine_math::camera::FpsCamera;
use std::env;
use std::sync::Arc;
use winit::{EventsLoop, WindowBuilder};

mod input;
mod render;

use input::Manager;

#[derive(Copy, Clone, Debug, PartialEq)]
enum EventResult {
    None,
    SurfaceLost,
    Closing,
}

fn handle_events(world: &mut World, event_loop: &mut EventsLoop, gilrs: &mut Gilrs) -> EventResult {
    let mut input_manager = world.get_resource_mut::<input::Manager>();
    let mut is_closing = false;
    let mut is_surface_lost = false;

    // poll window events
    {
        use winit::{Event, VirtualKeyCode, WindowEvent};
        event_loop.poll_events(|event| {
            input_manager.handle_winit_events(&event);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => is_closing = true,
                /*Event::WindowEvent { event:WindowEvent::Resized { .. }} => {
                    release_graph = true;
                }*/
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => is_closing = true,
                        Some(VirtualKeyCode::F11) => is_surface_lost = true,
                        _ => {}
                    }
                }                
                _ => {}
            }
        });
    }

    // poll gil events
    if !is_closing {
        use gilrs::Event;
        while let Some(event) = gilrs.next_event() {
            input_manager.handle_gil_events(&event);

            let Event { id, event, time } = event;
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
    world.register_resource_with(input::Manager::new());
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
            let inputManager = world.get_resource::<input::Manager>();
            let inputState = inputManager.get_state();
            let mut frame = world.get_resource_mut::<render::FrameInfo>();
            frame.frame_id += 1;

            let mut cam = world.get_resource_mut::<FpsCamera>();

            cam.move_up(inputState.get_joystick(0) * 0.001);
            cam.move_forward(inputState.get_joystick(0) * 0.01);
            cam.move_side(inputState.get_joystick(0) * 0.001);
            cam.yaw(-inputState.get_joystick(0) * 0.001);
            cam.pitch(inputState.get_joystick(0) * 0.001);
            cam.roll(inputState.get_joystick(0) * 0.001);
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

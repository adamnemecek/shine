#![feature(core_intrinsics)]

use gilrs::Gilrs;
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::{ResourceWorld, World};
use shine_math::camera::FpsCamera;
use std::env;
use std::sync::Arc;
use std::time::Instant;
use winit::{EventsLoop, WindowBuilder};

mod input;
use self::input::*;
mod components;
mod demo;
mod render;

#[derive(Copy, Clone, Debug, PartialEq)]
enum EventResult {
    None,
    SurfaceLost,
    Closing,
}

fn handle_events(world: &mut World, event_loop: &mut EventsLoop, gilrs: &mut Gilrs) -> EventResult {
    let mut input_manager = world.resource_mut::<InputManager>();
    let mut is_closing = false;
    let mut is_surface_lost = false;

    input_manager.prepare();
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
                } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => is_closing = true,
                    Some(VirtualKeyCode::F11) => is_surface_lost = true,
                    _ => {}
                },
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

    input_manager.update();

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
        .filter_module("shine_input", log::LevelFilter::Warn)
        .filter_module("shine", log::LevelFilter::Trace)
        .init();

    log::trace!("current executable {:?}", env::current_exe());
    log::trace!("current path {:?}", env::current_dir());

    let mut world = World::new();
    world.register_resource_with(FpsCamera::new());
    world.register_resource_with(input::create_input_manager());
    world.register_resource_with(render::FrameInfo::new());
    components::prepare_world(&mut world);

    let mut dispatch = demo::prepare_scene(&mut world);

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();

    let mut event_loop = EventsLoop::new();
    let mut gilrs = Gilrs::new().unwrap();

    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());
    let mut graph: Option<render::Graph> = None;
    let mut prev_frame_instant = None;
    let mut frame_count = 1;

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

        world.dispatch(&mut dispatch);

        let ellapsed_time = {
            match prev_frame_instant.replace(Instant::now()) {
                None => 0.0_f64,
                Some(prev) => prev.elapsed().as_micros() as f64,
            }
        };
        if ellapsed_time > 10000. {
            log::trace!("too long frame: {}", ellapsed_time);
        }
        frame_count += 1;

        {
            let mut frame = world.resource_mut::<render::FrameInfo>();
            frame.frame_id = frame_count;
            frame.ellapsed_time = ellapsed_time;

            let input_manager = world.resource::<InputManager>();
            let input_state = input_manager.get_state();
            let mut cam = world.resource_mut::<FpsCamera>();

            let dist = (ellapsed_time * 0.000001) as f32;
            let angle_dist = (ellapsed_time * 0.00001) as f32;

            cam.move_forward(input_state.get_button(buttons::MOVE_FORWARD) * dist);
            cam.move_side(input_state.get_button(buttons::MOVE_SIDE) * dist);
            cam.move_up(input_state.get_button(buttons::MOVE_UP) * dist);
            cam.yaw(input_state.get_button(buttons::YAW) * angle_dist);
            cam.roll(input_state.get_button(buttons::ROLL) * angle_dist);
            cam.pitch(input_state.get_button(buttons::PITCH) * angle_dist);
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

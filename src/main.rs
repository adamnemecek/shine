#![feature(core_intrinsics)]

use gilrs::Gilrs;
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::world::{ResourceWorld, World};
use shine_math::camera::FpsCamera;
use shine_stdext::time::{FrameLimit, FrameLimiter, FrameStatistics};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use winit::{EventsLoop, WindowBuilder};

mod input;
use self::input::*;
mod render;

mod logic;

mod voxel;

mod demo;

#[derive(Copy, Clone, Debug, PartialEq)]
enum EventResult {
    None,
    SurfaceLost,
    Closing,
}

fn handle_events(world: &World, event_loop: &mut EventsLoop, gilrs: &mut Gilrs) -> EventResult {
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

#[derive(Debug)]
struct SyncData {
    sync_count: usize,
}

type SyncLock = RwLock<SyncData>;

fn logic(world: &World, stopping: &AtomicBool, sync_lock: &SyncLock) {
    let mut dispatcher = demo::logic_tasks();
    let mut frame_limiter = FrameLimiter::new();

    while !stopping.load(Ordering::Relaxed) {
        frame_limiter.start();
        log::info!("logic update");
        //world.dispatch(&mut dispatcher);
        let _ = frame_limiter.limit(FrameLimit::SleepSpin(Duration::from_millis(100)));
        log::info!("logic frame limit: {:?}", frame_limiter);

        //sync point b/n render and logic
        {
            let mut l = sync_lock.write().unwrap();
            l.sync_count += 1;
            thread::sleep(Duration::from_micros(10));
        }
    }
}

fn render(world: &World, stopping: &AtomicBool, sync_lock: &SyncLock) {
    let mut dispatcher = demo::render_tasks();

    let mut event_loop = EventsLoop::new();
    let mut gilrs = Gilrs::new().unwrap();
    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());
    let mut frame_limiter = FrameLimiter::new();
    let mut frame_stats = FrameStatistics::new();

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();
    let mut graph: Option<render::Graph> = None;

    loop {
        frame_limiter.start();
        factory.maintain(&mut families);

        let event_result = handle_events(&world, &mut event_loop, &mut gilrs);

        if event_result == EventResult::SurfaceLost || event_result == EventResult::Closing {
            if let Some(graph) = graph.take() {
                graph.dispose(&mut factory, &world);
                render::dispose(&mut factory, &world);
            }
        }

        if event_result == EventResult::Closing {
            log::trace!("closing");
            stopping.store(true, Ordering::Relaxed);
            break;
        }

        {
            let r = sync_lock.read().unwrap();
            //log::trace!("r: {:?}", *r);
            world.dispatch(&mut dispatcher);
            frame_stats.end_frame();

            {
                let elapsed_time = frame_stats.get_last_frame_time().as_micros() as f32 / 1_000_000.0_f32;

                let mut frame = world.resource_mut::<render::FrameInfo>();
                frame.frame_id = frame_stats.get_frame_count();
                frame.elapsed_time = elapsed_time;

                let input_manager = world.resource::<InputManager>();
                let input_state = input_manager.get_state();
                let mut cam = world.resource_mut::<FpsCamera>();

                let dist = elapsed_time;
                let angle_dist = elapsed_time;

                cam.move_forward(input_state.get_button(buttons::MOVE_FORWARD) * dist);
                cam.move_side(input_state.get_button(buttons::MOVE_SIDE) * dist);
                cam.move_up(input_state.get_button(buttons::MOVE_UP) * dist);
                cam.yaw(input_state.get_button(buttons::YAW) * angle_dist);
                cam.roll(input_state.get_button(buttons::ROLL) * angle_dist);
                cam.pitch(input_state.get_button(buttons::PITCH) * angle_dist);
            }

            if graph.is_none() {
                let surface = factory.create_surface(window.clone());
                graph = Some(render::init(&mut factory, &mut families, surface, &world));
            }

            if let Some(ref mut graph) = graph {
                graph.run(&mut factory, &mut families, &world);
            }
        }

        /*let t = frame_limiter.limit(FrameLimit::SleepSpin(Duration::from_micros(10)));
        log::info!(
            "t: {:?}, {:?}, {:?}, {:?}, {:?}us",
            t,
            frame_limiter.work_time(),
            frame_limiter.sleep_time(),
            frame_limiter.spin_time(),
            frame_limiter.global_off_time_us()
        );*/
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

    let world = {
        let mut world = World::new();
        world.register_resource_with(FpsCamera::new());
        world.register_resource_with(input::create_input_manager());
        world.register_resource_with(render::FrameInfo::new());
        demo::prepare_world(&mut world);
        world
    };
    let stopping = AtomicBool::new(false);
    let sync_lock = RwLock::new(SyncData { sync_count: 0 });

    crossbeam::scope(|scope| {
        let _logic_thread = scope.spawn(|_| {
            logic(&world, &stopping, &sync_lock);
        });
        let _render_thread = scope.spawn(|_| {
            render(&world, &stopping, &sync_lock);
        });
    })
    .unwrap();

    log::trace!("bye.");
}

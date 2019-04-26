#![feature(core_intrinsics)]

use gilrs::Gilrs;
use interact_prompt;
use parking_lot::{RwLock, RwLockReadGuard};
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::world::{EntityWorld, ResourceWorld, World};
use shine_shard::camera::{FpsCamera, RawCamera, RenderCamera};
use shine_stdext::time::{FrameLimit, FrameLimiter, FrameStatistics};
use std::env;
use std::sync::{Arc, Weak};
use std::thread;
use std::time::Duration;
use winit::{EventsLoop, WindowBuilder};

mod input;
use self::input::*;
mod render;

mod logic;

mod voxel;

mod demo;
use demo::{App, AppLogic, AppRender, Demo};

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

fn logic<A: App>(app: &A, logic_world: &mut World, render_world: &RwLock<World>, stop_signal: Weak<()>) {
    let mut frame_limiter = FrameLimiter::new();
    let mut app = app.create_logic();

    while stop_signal.upgrade().is_some() {
        frame_limiter.start();
        log::info!("logic update");
        app.update(logic_world);
        let _ = frame_limiter.limit(FrameLimit::SleepSpin(Duration::from_millis(100)));
        log::info!("logic frame limit: {:?}", frame_limiter);

        //sync point b/n render and logic
        {
            let mut render_world = render_world.write();
            log::info!("sync render to logic");
            app.sync(logic_world, &mut *render_world);
        }
    }
}

fn render<A: App>(app: &A, world: &RwLock<World>, mut stop_signal: Option<Arc<()>>) {
    let mut event_loop = EventsLoop::new();
    let mut gilrs = Gilrs::new().unwrap();
    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());
    let mut frame_limiter = FrameLimiter::new();
    let mut frame_stats = FrameStatistics::new();

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();
    let mut graph: Option<render::Graph> = None;

    let mut app = app.create_render();

    loop {
        //frame_stats.start_frame();
        frame_limiter.start();
        factory.maintain(&mut families);
        let world = world.read();

        let event_result = handle_events(&world, &mut event_loop, &mut gilrs);

        if event_result == EventResult::SurfaceLost || event_result == EventResult::Closing {
            if let Some(graph) = graph.take() {
                graph.dispose(&mut factory, &world);
                render::dispose(&mut factory, &world);
            }
        }

        if event_result == EventResult::Closing {
            log::trace!("closing");
            stop_signal.take();
            break;
        }

        app.update(&*world);

        if graph.is_none() {
            let surface = factory.create_surface(window.clone());
            graph = Some(render::init(&mut factory, &mut families, surface, &world));
        }

        if let Some(ref mut graph) = graph {
            graph.run(&mut factory, &mut families, &world);
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
        frame_stats.end_frame();

        RwLockReadGuard::unlock_fair(world);
    }
}

struct InteractHandler {
    stop_signal: Weak<()>,
}

impl interact_prompt::Handler for InteractHandler {
    fn receive_interaction(&self, intr: interact_prompt::Interaction) -> interact_prompt::Response {
        if self.stop_signal.upgrade().is_none() {
            interact_prompt::Response::Exit
        } else {
            match intr {
                /*interact_prompt::Interaction::Line(string) => {
                    interact_prompt::Commands::new().handle_cmd(&string);
                }*/
                _ => {}
            }
            interact_prompt::Response::Continue
        }
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

    let mut logic_world = {
        log::trace!("prepare logic world");
        let mut world = World::new();
        world.register_resource_with(RawCamera::new());
        world
    };

    let mut render_world = {
        log::trace!("prepare render world");
        let mut world = World::new();
        world.register_resource_with(input::create_input_manager());
        world.register_resource_with(render::FrameInfo::new());
        world.register_resource_with(RenderCamera::new());
        world.register_resource_with(FpsCamera::new());
        world
    };

    let app = Demo::default();
    app.prepare_logic(&mut logic_world);
    app.prepare_render(&mut logic_world, &mut render_world);
    
    let render_world = RwLock::new(render_world);

    let stop_signal = Arc::new(());
    let stop_observer = Arc::downgrade(&stop_signal);

    crossbeam::scope(|scope| {
        let _logic_thread = scope.builder().name("logic".to_string()).spawn(|_| {
            logic(&app, &mut logic_world, &render_world, stop_observer.clone());
        });
        let _render_thread = scope.builder().name("render".to_string()).spawn(|_| {
            render(&app, &render_world, Some(stop_signal));
        });
        /*let _interact_thread = scope.builder().name("interact".to_string()).spawn(|_| {
            interact_prompt::direct(interact_prompt::Settings::default(), InteractHandler{stop_signal:stop_observer.clone()}).unwrap();
        });*/
    })
    .unwrap();

    log::trace!("bye.");
}

#![feature(core_intrinsics)]

use gilrs::Gilrs;
use interact_prompt;
use parking_lot::{RwLock, RwLockWriteGuard};
use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::world::{ResourceWorld, World};
use shine_stdext::time::{FrameLimit, FrameLimiter};
use std::env;
use std::sync::{Arc, Weak};
use std::time::Duration;
use winit::{EventsLoop, WindowBuilder};

mod app;
mod demo;
mod input;
mod logic;
mod render;
mod voxel;

use self::app::{App, AppLogicHandler, AppRenderHandler};
use self::input::*;
use demo::Demo;

#[derive(Copy, Clone, Debug, PartialEq)]
enum EventResult {
    None,
    SurfaceLost,
    Closing,
}

struct AppLogic {
    world: World,
}

struct AppRender {
    world: World,
}

fn logic<A: App>(app: &A, app_logic: &RwLock<AppLogic>, app_render: &RwLock<AppRender>, stop_signal: Weak<()>) {
    let mut frame_limiter = FrameLimiter::new();
    let mut app = app.create_logic_handler();

    while stop_signal.upgrade().is_some() {
        let world_frame_length = {
            log::info!("logic update");
            frame_limiter.start();

            let mut app_logic = app_logic.write();
            let logic_world = &mut app_logic.world;

            app.update(logic_world);
            let world_frame_length = logic_world.resource::<logic::LogicConfig>().world_frame_length();

            RwLockWriteGuard::unlock_fair(app_logic);

            let _ = frame_limiter.limit(FrameLimit::SleepSpin(world_frame_length));

            world_frame_length
        };

        {
            log::info!("sync render to logic");
            let mut app_logic = app_logic.write();
            let mut app_render = app_render.write();

            let logic_world = &mut app_logic.world;
            let render_world = &mut app_render.world;

            render_world.resource_mut::<render::FrameInfo>().start_logic(world_frame_length);
            app.sync(logic_world, &mut *render_world);

            RwLockWriteGuard::unlock_fair(app_render);
            RwLockWriteGuard::unlock_fair(app_logic);
        }
    }
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

fn render<A: App>(app: &A, app_render: &RwLock<AppRender>, mut stop_signal: Option<Arc<()>>) {
    let mut event_loop = EventsLoop::new();
    let mut gilrs = Gilrs::new().unwrap();
    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();
    let mut graph: Option<render::Graph> = None;

    let mut app = app.create_render_handler();
    let mut frame_limiter = FrameLimiter::new();

    loop {
        frame_limiter.start();
        factory.maintain(&mut families);

        {
            let mut app_render = app_render.write();
            let world = &mut app_render.world;

            {
                let mut frame_info = world.resource_mut::<render::FrameInfo>();
                frame_info.start_frame();
                log::info!("{:#?}", &*frame_info);
            }

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

            app.update(world);

            if graph.is_none() {
                let surface = factory.create_surface(window.clone());
                graph = Some(render::init(&mut factory, &mut families, surface, &world));
            }

            if let Some(ref mut graph) = graph {
                graph.run(&mut factory, &mut families, &world);
            }
            RwLockWriteGuard::unlock_fair(app_render);
        }

        let _ = frame_limiter.limit(FrameLimit::SleepSpin(Duration::from_millis(10)));
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

    let app = Demo::default();

    let mut logic_world = World::new();
    let logic_config = app.create_logic_config();
    let frame_world_length = logic_config.world_frame_length();
    logic_world.register_resource_with(logic_config);
    app.prepare_logic(&mut logic_world);

    // todo: start render thread after the 1st sync point, thus no need to configure anything prior
    let mut render_world = World::new();
    render_world.register_resource_with(input::create_input_manager());
    render_world.register_resource_with(render::FrameInfo::new(frame_world_length));
    app.prepare_render(&mut logic_world, &mut render_world);

    let app_logic = RwLock::new(AppLogic { world: logic_world });
    let app_render = RwLock::new(AppRender { world: render_world });

    let stop_signal = Arc::new(());
    let stop_observer = Arc::downgrade(&stop_signal);

    crossbeam::scope(|scope| {
        let _logic_thread = scope.builder().name("logic".to_string()).spawn(|_| {
            logic(&app, &app_logic, &app_render, stop_observer.clone());
        });
        let _render_thread = scope.builder().name("render".to_string()).spawn(|_| {
            render(&app, &app_render, Some(stop_signal));
        });
        /*let _interact_thread = scope.builder().name("interact".to_string()).spawn(|_| {
            interact_prompt::direct(
                interact_prompt::Settings::default(),
                InteractHandler {
                    stop_signal: stop_observer.clone(),
                },
            )
            .unwrap();
        });*/
    })
    .unwrap();

    log::trace!("bye.");
}

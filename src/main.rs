#![feature(core_intrinsics)]

use rendy::factory::{Config as RendyConfig, Factory};
use shine_ecs::{ResourceWorld, World};
use shine_math::camera::FreeCamera;
use std::env;
use std::sync::Arc;
use winit::{Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent};

mod render;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("shine", log::LevelFilter::Trace)
        .init();

    log::trace!("current executable {:?}", env::current_exe());
    log::trace!("current path {:?}", env::current_dir());
    let mut world = World::new();
    world.register_resource_with(FreeCamera::new());

    let config: RendyConfig = Default::default();
    let (mut factory, mut families): (Factory<render::Backend>, _) = rendy::factory::init(config).unwrap();
    let mut event_loop = EventsLoop::new();

    let window = Arc::new(WindowBuilder::new().with_title("Shine").build(&event_loop).unwrap());
    let mut graph: Option<render::Graph> = None;

    let mut is_closing = false;
    loop {
        let mut release_graph = false;
        factory.maintain(&mut families);
        event_loop.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => is_closing = true,
                /*WindowEvent::Resized { .. } => {
                    release_graph = true;
                }*/
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        is_closing = true;
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::F11) {
                        release_graph = true;
                    }
                }
                _ => {}
            },
            _ => {}
        });

        if release_graph || is_closing {
            if let Some(graph) = graph.take() {
                graph.dispose(&mut factory, &mut world);
                render::dispose(&mut factory, &mut world);
            }
        }

        if is_closing {
            log::trace!("closing");
            break;
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

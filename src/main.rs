#![feature(trace_macros)]

#[macro_use]
extern crate dragorust_engine;

use std::time::Duration;

mod world;
mod view;

pub use dragorust_engine::*;
use world::*;
use view::*;

pub fn main() {
    let mut engine = render::Engine::new().expect("Could not initialize render engine");
    let mut world = World::new();

    let mut window = render::WindowSettings::new()
        .title("main")
        .size((1024, 1024))
        .build(&mut engine).expect("Could not initialize main window");
    let view: View = world.create_view(&mut window);

    let mut sub_window = render::WindowSettings::new()
        .title("sub")
        .size((100, 100))
        //.gl_profile(render::OpenGLProfile::ES2)
        .build(&mut engine).expect("Could not initialize sub window");
    let sub_view = world.create_view(&mut sub_window);

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        world.update();

        view.update();
        sub_view.update();

        window.render().unwrap();
        sub_window.render().unwrap();
    }
}

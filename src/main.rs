#![feature(trace_macros)]

#[macro_use]
extern crate dragorust_engine;

use std::time::Duration;

mod world;
mod viewdata;

pub use dragorust_engine::*;
use world::*;
use viewdata::*;

pub fn main() {
    let mut engine = render::Engine::new().expect("Could not initialize render engine");

    let world = WorldWrapper::new();

    let mut window = render::WindowSettings::new()
        .title("main")
        .size((1024, 1024))
        .build(&mut engine).expect("Could not initialize main window");
    let window_data: ViewDataWrapper = ViewDataWrapper::new(world.clone());
    window.set_surface_handler(window_data.clone());
    window.set_input_handler(window_data.clone());


    let mut sub_window = render::WindowSettings::new()
        .title("sub")
        .size((100, 100))
        //.gl_profile(render::OpenGLProfile::ES2)
        .build(&mut engine).expect("Could not initialize main window");
    let sub_window_data = ViewDataWrapper::new(world.clone());
    sub_window.set_surface_handler(sub_window_data.clone());
    sub_window.set_input_handler(sub_window_data.clone());


    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        if !window.is_closed() {
            window_data.update();

            window.start_render().unwrap();
            window_data.render(&mut window);
            window.end_render().unwrap();
        }

        if !sub_window.is_closed() {
            sub_window_data.update();

            sub_window.start_render().unwrap();
            sub_window_data.render(&mut sub_window);
            sub_window.end_render().unwrap();
        }
    }
}

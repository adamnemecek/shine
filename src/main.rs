#![feature(trace_macros)]

extern crate dragorust_engine;

use std::time::Duration;

mod world;
mod viewdata;

pub use dragorust_engine::*;
use world::*;
use viewdata::*;


trace_macros!(true);
vertex_declaration! { VxLine, VxLineLocation,
    attributes {
        position: render::Float32x4,
        normal: render::Float32x3 = f32x3!(0, 0, 1),
    }
}
trace_macros!(false);

pub fn main() {
    println!("{:?}", VxLine::get_declaration(1));

    let mut engine = render::Engine::new().expect("Could not initialize render engine");

    let world = WorldWrapper::new();

    let mut window = render::WindowSettings::new()
        .title("main")
        .size((1024u32, 1024u32))
        .build(&mut engine).expect("Could not initialize main window");
    let window_data: ViewDataWrapper = ViewDataWrapper::new(world.clone());
    window.set_surface_handler(window_data.clone());
    window.set_input_handler(window_data.clone());


    let mut sub_window = render::WindowSettings::new()
        .title("sub")
        .size((100u32, 100u32))
        .build(&mut engine).expect("Could not initialize main window");
    let sub_window_data = ViewDataWrapper::new(world.clone());
    sub_window.set_surface_handler(sub_window_data.clone());
    sub_window.set_input_handler(sub_window_data.clone());


    let mut t = 0.;
    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }
        t = t + 0.01;
        if t > 1. {
            engine.quit();
            t = 0.;
        }

        if !window.is_closed() {
            window.start_render().unwrap();
            window.hello_world(t);

            let ref mut view_data = *window_data.borrow_mut();
            view_data.update();
            window.process_queue(&mut view_data.render_queue).unwrap();

            window.end_render().unwrap();
        }

        if !sub_window.is_closed() {
            sub_window.start_render().unwrap();
            window.hello_world(t);

            let ref mut view_data = *sub_window_data.borrow_mut();
            view_data.update();
            sub_window.process_queue(&mut view_data.render_queue).unwrap();

            sub_window.end_render().unwrap();
        }
    }
}

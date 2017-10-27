#[macro_use]
extern crate dragorust_engine;

use std::time::Duration;

mod world;
mod view;

use std::rc::Rc;
use std::cell::RefCell;
pub use dragorust_engine::*;
use render::*;
use world::*;
use view::*;

pub fn main() {
    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");
    let world = Rc::new(RefCell::new(World::new()));

    let mut window = render::WindowSettings::new()
        .title("main")
        .size((102, 102))
        .build(&engine, SimpleView::new(world.clone())).expect("Could not initialize main window");

    let mut sub_window = render::WindowSettings::new()
        .title("sub")
        .size((100, 100))
        //.gl_profile(render::OpenGLProfile::ES2)
        .build(&engine, SimpleView::new(world.clone())).expect("Could not initialize sub window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        world.borrow_mut().update();

        window.update_view();
        sub_window.update_view();

        window.render().unwrap();
        sub_window.render().unwrap();
    }
}

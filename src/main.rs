#[macro_use]
extern crate dragorust_engine;

use std::time::Duration;

mod game;
mod view;

pub use dragorust_engine::*;
use render::*;
use view::*;

pub fn main() {
    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");
    let game = game::create();

    let mut window = render::WindowSettings::new()
        .title("main")
        .size((102, 102))
        .build(&engine, SimpleView::new(game.clone())).expect("Could not initialize main window");

    let mut sub_window = render::WindowSettings::new()
        .title("sub")
        .size((100, 100))
        //.gl_profile(render::OpenGLProfile::ES2)
        .build(&engine, SimpleView::new(game.clone())).expect("Could not initialize sub window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        game.borrow_mut().update();

        window.update_view();
        sub_window.update_view();

        window.render().unwrap();
        sub_window.render().unwrap();
    }
}

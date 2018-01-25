//#[macro_use]
//extern crate dragorust_render_gl as render;
//extern crate dragorust_world as world;


//use std::time::Duration;

//mod game;
//mod view;

//use render::*;
//use view::*;


pub fn main() {
/*    let engine = render::opengl::PlatformEngine::new().expect("Could not initialize render engine");
//    let game = game::create();

    let mut window = engine.window_builder()/*render::PlatformWindowSettings::default()*/
        .title("main")
        .size((1024, 1024))
        .build(&engine, SimpleView::new(game.clone())).expect("Could not initialize main window");

    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((256, 256))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
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
    }*/
}

mod container;
mod render;

pub use render::{RenderEngine, RenderWindow};

fn main() {
    let mut render_engine = render::Engine::new().expect("");
    let mut main_window = render_engine.create_window(1024, 1024, "alma").expect("");

    while !main_window.is_closed() {
        main_window.handle_message(None);

        main_window.render_start();
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        main_window.render_start();
    }
    //let (p, c) = spsc::state_channel::<i32>();
}

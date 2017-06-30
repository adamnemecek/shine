extern crate gl;

mod container;
mod render;

use render::{IEngine, IWindow};

fn main() {
    let mut render_engine = render::create_engine().expect("Could not initialize render engine");
    let mut main_window = render_engine.create_window(1024, 1024, "alma").expect("Could not initialize main window");
    let mut sub_window = render_engine.create_window(100, 100, "sub_alma").expect("Could not initialize secondary window");

    while main_window.handle_message(None) {
        if main_window.render_start().is_ok() {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            main_window.render_end().unwrap();
        }

        if sub_window.handle_message(None) {
            if sub_window.render_start().is_ok() {
                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                sub_window.render_end().unwrap();
            }
        }
    }

    drop(main_window);
    while sub_window.handle_message(None) {
        if sub_window.render_start().is_ok() {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            sub_window.render_end().unwrap();
        }
    }
}

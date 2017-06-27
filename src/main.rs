extern crate gl;

mod container;
mod render;

use render::{RenderEngine, RenderWindow, WindowHandle};

fn main() {
    let sub_window:WindowHandle;

    {
        let mut render_engine = render::create_engine().expect("Could not initialize render engine");
        let main_window = render_engine.create_window(1024, 1024, "alma").expect("Could not initialize main window");
        sub_window = render_engine.create_window(100, 100, "sub_alma").expect("Could not initialize sub window");

        while main_window.borrow_mut().handle_message(None) {
            if main_window.borrow_mut().render_start().is_ok() {
                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                main_window.borrow_mut().render_end().unwrap();
            }

            if sub_window.borrow_mut().render_start().is_ok() {
                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                sub_window.borrow_mut().render_end().unwrap();
            }
        }
    }

    loop {
        if sub_window.borrow_mut().render_start().is_ok() {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            sub_window.borrow_mut().render_end().unwrap();
        }
    }
    //let (p, c) = spsc::state_channel::<i32>();
}

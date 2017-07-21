extern crate gl;

mod container;
mod render;

use std::rc::{Rc};
use std::cell::{RefCell};
use render::*;

struct World {
    shader: ShaderProgram,
    alma: i32,
}

impl World {
    fn new() -> World {
        World {
            shader: ShaderProgram::new(),
            alma: 1,
        }
    }
}

impl Drop for World {
    fn drop(&mut self) {
        println!("dropping world");
    }
}


struct SurfaceHandler {
    world: Rc<RefCell<World>>,
}

impl Drop for SurfaceHandler {
    fn drop(&mut self) {
        println!("SurfaceHandler  dropped");
    }
}

impl ISurfaceHandler for SurfaceHandler {
    fn on_ready(&mut self, win: &mut Window) {
        println!("on_surface_ready");
        let sh_source = [(0, "a"), (0, "b")];
        win.render_process(|ref mut ll| {
            self.world.borrow_mut().shader.create_program(ll, sh_source.iter());
        }).unwrap();
    }

    fn on_lost(&mut self, win: &mut Window) {
        println!("on_surface_lost");
        win.render_process(|ref mut ll| {
            self.world.borrow_mut().shader.release(ll);
        }).unwrap();
    }
}


fn main() {
    println!("main");
    let mut render_engine = render::create_engine().expect("Could not initialize render engine");
    let mut main_window = render_engine.create_window(1024, 1024, "alma").expect("Could not initialize main window");
    let mut sub_window = render_engine.create_window(100, 100, "sub_alma").expect("Could not initialize secondary window");

    let world = Rc::new(RefCell::new(World::new()));
    main_window.set_surface_handler(SurfaceHandler { world: world.clone() });
    world.borrow_mut().alma = 1;

    println!("main window loop");
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

    println!("close all window");
    render_engine.close_all_windows();

    drop(main_window);

    println!("sub window loop");
    while sub_window.handle_message(None) {
        if sub_window.render_start().is_ok() {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            sub_window.render_end().unwrap();
        }
    }

    println!("main end");
}

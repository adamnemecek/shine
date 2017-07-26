extern crate gl;

mod container;
mod render;

use std::rc::{Rc};
use std::cell::{RefCell};
use render::*;

struct World {
    render_queue: CommandQueue,
    shader: ShaderProgram,
}

impl World {
    fn new() -> World {
        World {
            render_queue: CommandQueue::new(),
            shader: ShaderProgram::new(),
        }
    }
}


struct SurfaceHandler {
    world: Rc<RefCell<World>>,
}

impl ISurfaceHandler for SurfaceHandler {
    fn on_ready(&mut self) {
        println!("on_surface_ready");
        let sh_source = [(ShaderType::VertexShader, "vert"), (ShaderType::FragmentShader, "frag")];
        let world = &mut *self.world.borrow_mut();
        world.shader.set_sources(&mut world.render_queue, sh_source.iter());
    }

    fn on_lost(&mut self) {
        println!("on_surface_lost");
        let world = &mut *self.world.borrow_mut();
        world.shader.release(&mut world.render_queue);
    }
}


fn main() {
    println!("main");
    let mut render_engine = render::create_engine().expect("Could not initialize render engine");
    let main_window = render_engine.create_window(1024, 1024, "alma").expect("Could not initialize main window");
    let sub_window = render_engine.create_window(100, 100, "sub_alma").expect("Could not initialize secondary window");

    let world = Rc::new(RefCell::new(World::new()));
    main_window.set_surface_handler(SurfaceHandler { world: world.clone() }).unwrap();
    //sub_window.set_surface_handler(SurfaceHandler { world: world.clone() }).unwrap();

    println!("window loop");
    while render_engine.handle_message(None) {
        if main_window.is_open() {
            if main_window.start_render().is_ok() {
                unsafe {
                    gl::ClearColor(0.2, 0.5, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                main_window.process_queue(&mut world.borrow_mut().render_queue).unwrap();
                main_window.end_render().unwrap();
            }
        }

        if sub_window.is_open() {
            if sub_window.start_render().is_ok() {
                unsafe {
                    gl::ClearColor(0.5, 0.2, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                sub_window.end_render().unwrap();
            }
        }
    }
}

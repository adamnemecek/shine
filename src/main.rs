pub mod container;
#[macro_use]
pub mod render;

mod renderer;
mod world;

use std::rc::{Rc};
use std::cell::{RefCell};

pub use render::*;
use world::*;
//use renderer::*;

/*vertex_declaration!(Alma{
    position: Float32x4 = f32x4!(),
    color: Float32x2 = f32x2!(0.,1.),
});

fn foo() {
    let a = Alma::Vertex::new();
    println!("{}", a);
    println!("{:?}", a.position);

    let mut buf = Alma::new_static_source(10);
    buf[0] = Alma::Vertex { position: f32x4!(30), color: Float32x2::new(41., 34.) };
    buf[1] = a;

    println!("{}, {}, {}", Alma::Location::position, Alma::Location::color, Alma::Location::Count);
    println!("{}", Alma::get_location_by_name("position"));
    println!("{}", Alma::get_location_by_name("color"));
    println!("{}", Alma::get_location_by_name("sdf"));

    println!("{:?}", buf[0].position);
}
*/

struct SurfaceHandler {
    //world: Rc<RefCell<World>>,
    //shader: ShaderProgram,
}

impl render::SurfaceHandler for SurfaceHandler {
    fn on_ready(&mut self, window: &Window) {
        println!("on_ready");
        /* let sh_source = [(ShaderType::VertexShader, r#"
 attribute vec4 vPosition;
 void main()
 {
     gl_Position = vPosition;
 }"#
         ), (ShaderType::FragmentShader, r#"
 void main()
 {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
 }"#
         )];
         let render = &mut self.world.render.borrow_mut();*/

        // self.shader.set_sources(&mut render.render_queue, sh_source.iter());
        // window.process_single_queue(&mut render.render_queue).unwrap();
    }

    #[allow(unused_variables)]
    fn on_lost(&mut self, window: &Window) {
        println!("on_lost");
        /*let render = &mut self.world.render.borrow_mut();

        self.shader.release(&mut render.render_queue);
        window.process_single_queue(&mut render.render_queue).unwrap();*/
    }
}

struct InputHandler();

impl render::InputHandler for InputHandler {
    fn on_key(&mut self, window: &Window) {
        window.close()
    }
}

fn main() {
    let world = Rc::new(RefCell::new(World::new()));

    let render_engine = Engine::new().expect("Could not initialize render engine");
    let main_window = Window::new(&render_engine, 1024, 1024, "main").expect("Could not initialize main window");
    main_window.set_input_handler(InputHandler()).unwrap();
    main_window.set_surface_handler(SurfaceHandler { /*world: world.clone()*/ /*,shader: ShaderProgram::new()*/ }).unwrap();

    while render_engine.handle_message(None) {
        if main_window.is_open() {
            //            let render = &mut *world.render.borrow_mut();

            if main_window.start_render().is_ok() {
                //unsafe {
                //  gl::ClearColor(0.2, 0.5, 0.2, 1.0);
                //gl::Clear(gl::COLOR_BUFFER_BIT);
                //}
                //                main_window.process_queue(&mut render.render_queue).unwrap();
                main_window.end_render().unwrap();
            }
        }
    }
}

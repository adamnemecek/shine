extern crate gl;

pub mod container;
#[macro_use]
pub mod render;
pub use render::*;

mod renderer;
mod world;

use world::World;
//use renderer::Render;

vertex_declaration!(Alma{
    position: Float32x4 = f32x4!(),
    color: Float32x2 = f32x2!(0.,1.),
});

fn foo() {
    let a = Alma::Vertex::new();
    println!("{}", a);
    println!("{:?}", a.position);

    let mut buf = Alma::StaticSource::new(10);
    buf.push(Alma::Vertex { position: f32x4!(30), color: Float32x2::new(41., 34.) });
    buf.push(a);

    println!("{}, {}, {}", Alma::Location::position, Alma::Location::color, Alma::Location::Count);
    println!("{}", Alma::get_location_by_name("position"));
    println!("{}", Alma::get_location_by_name("color"));
    println!("{}", Alma::get_location_by_name("sdf"));

    println!("{:?}", buf[0].position);
}


struct SurfaceHandler {
    world: World,
    shader: ShaderProgram,
}

impl ISurfaceHandler for SurfaceHandler {
    fn on_ready(&mut self, window: &Window) {
        let sh_source = [(ShaderType::VertexShader, r#"
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
        let render = &mut self.world.render.borrow_mut();

        self.shader.set_sources(&mut render.render_queue, sh_source.iter());
        window.process_single_queue(&mut render.render_queue).unwrap();
    }

    #[allow(unused_variables)]
    fn on_lost(&mut self, window: &Window) {
        let render = &mut self.world.render.borrow_mut();

        self.shader.release(&mut render.render_queue);
        window.process_single_queue(&mut render.render_queue).unwrap();
    }
}

fn main() {
    foo();
    let world = World::new();

    let mut render_engine = render::create_engine().expect("Could not initialize render engine");
    let main_window = render_engine.create_window(1024, 1024, "main").expect("Could not initialize main window");
    main_window.set_surface_handler(SurfaceHandler {
        world: world.clone(),
        shader: ShaderProgram::new(),
    }).unwrap();

    while render_engine.handle_message(None) {
        if main_window.is_open() {
            let render = &mut *world.render.borrow_mut();

            if main_window.start_render().is_ok() {
                unsafe {
                    gl::ClearColor(0.2, 0.5, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                main_window.process_queue(&mut render.render_queue).unwrap();
                main_window.end_render().unwrap();
            }
        }
    }
}

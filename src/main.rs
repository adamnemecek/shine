extern crate gl;

pub mod container;
pub mod render;
pub use render::*;

mod renderer;
mod world;
mod foo;

use world::World;
//use renderer::Render;



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
    foo::do_foo();
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

pub mod container;
#[macro_use]
pub mod render;

pub use render::*;

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

fn on_surface_ready(window: &mut Window, queue: &mut CommandQueue) {
    println!("on_ready");
    /*
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
    shader.set_sources(&mut queue, sh_source.iter());
    window.process_single_queue(&mut queue).unwrap();*/
}

fn on_surface_lost(window: &mut Window, queue: &mut CommandQueue) {
    println!("on_lost");
    /*shader.release(&mut queue);
    window.process_single_queue(&mut queue).unwrap();*/
}


fn render_frame(window: &mut Window, queue: &mut CommandQueue) {
    window.start_render().unwrap();
    //unsafe {
    //  gl::ClearColor(0.2, 0.5, 0.2, 1.0);
    //  gl::Clear(gl::COLOR_BUFFER_BIT);
    //}
    window.process_queue(queue).unwrap();
    window.end_render().unwrap();
}

fn main() {
    render::Engine::init().expect("Could not initialize render engine");

    let mut window = WindowSettings::new()
        .title("main")
        .size((1024u32, 1024u32))
        .build().expect("Could not initialize main window");
    let mut render_queue = render::CommandQueue::new();

    let mut is_running = true;
    while is_running {
        //let render = &mut *world.render.borrow_mut();

        let event = window.wait_event();
        match event {
            render::Event::SurfaceReady => { on_surface_ready(&mut window, &mut render_queue) },
            render::Event::SurfaceLost => { on_surface_lost(&mut window, &mut render_queue) },
            render::Event::Closed => { is_running = false; },
            _ => {}
        }

        if !window.is_closed() {
            render_frame(&mut window, &mut render_queue);
        }
    }

    render::Engine::shutdown();
}

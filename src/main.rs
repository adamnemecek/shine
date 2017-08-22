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

/*
static sh_source = [
    (ShaderType::VertexShader, r#"
 attribute vec4 vPosition;
 void main()
 {
     gl_Position = vPosition;
 }"#),
    (ShaderType::FragmentShader, r#"
 void main()
 {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
 }"#)];
*/

struct PerViewData {}

impl PerViewData {
    fn new() -> PerViewData {
        PerViewData {}
    }
}

struct SurfaceHandler {
    view_data: Option<PerViewData>
}

impl SurfaceHandler {
    fn new() -> SurfaceHandler {
        SurfaceHandler {
            view_data: None
        }
    }
}

impl SurfaceEventHandler for SurfaceHandler {
    fn on_ready(&mut self, window: &mut Window) {
        println!("on_ready");
        assert!(self.view_data.is_none());
        self.view_data = Some(PerViewData::new());
        //shader.set_sources(&mut queue, sh_source.iter());
        //window.process_single_queue(&mut queue).unwrap();
    }

    fn on_lost(&mut self, window: &mut Window) {
        println!("on_lost");
        assert!(self.view_data.is_some());
        //shader.release(&mut queue);
        //window.process_single_queue(&mut queue).unwrap();
        self.view_data = None;
    }
}

impl Clone for SurfaceHandler {
    fn clone(&self) -> SurfaceHandler {
        SurfaceHandler {
            // view data cannot be cloned, it have to be regenerated
            view_data: None,
        }
    }
}


fn main() {
    let mut engine = render::Engine::new().expect("Could not initialize render engine");
    let mut window = WindowSettings::new()
        .title("main")
        .size((1024u32, 1024u32))
        .build(&mut engine).expect("Could not initialize main window");
    window.set_surface_handler(SurfaceHandler::new());

    let mut sub_window = WindowSettings::new()
        .title("sub")
        .size((100u32, 100u32))
        .build(&mut engine).expect("Could not initialize main window");
    sub_window.set_surface_handler(SurfaceHandler::new());

    let mut render_queue = render::CommandQueue::new();

    let mut t = 0.;
    loop {
        if !engine.dispatch_event(DispatchTimeout::Immediate) {
            break;
        }
        t = t + 0.01;
        if t > 1. { t = 0.; }

        if !window.is_closed() {
            window.start_render().unwrap();
            unsafe {
                gl::ClearColor(0.2, t, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            //window.process_queue(queue).unwrap();
            window.end_render().unwrap();
        }
    }
}

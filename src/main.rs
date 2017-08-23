pub mod container;
#[macro_use]
pub mod render;

use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use render::*;

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

struct Data {}

impl Data {
    fn new() -> Data {
        Data {}
    }
}


struct ViewData {
    data: Rc<RefCell<Data>>,
    render_queue: CommandQueue,
    shader: ShaderProgram,
}

impl ViewData {
    fn new(data: Rc<RefCell<Data>>) -> ViewData {
        ViewData {
            data: data,
            render_queue: CommandQueue::new(),
            shader: ShaderProgram::new(),
        }
    }

    fn update(&mut self) {}
}

#[derive(Clone)]
struct ViewDataWrapper(Rc<RefCell<ViewData>>);

impl SurfaceEventHandler for ViewDataWrapper {
    fn on_ready(&mut self, window: &mut Window) {
        println!("on_ready");
        let ref mut view_data = *self.0.borrow_mut();
        let queue = &mut view_data.render_queue;

        //view_data.shader.set_sources(&mut view_data.queue, sh_source.iter());

        window.start_render().unwrap();
        window.process_queue(queue).unwrap();
        window.end_render().unwrap();
    }

    fn on_lost(&mut self, window: &mut Window) {
        println!("on_lost");
        let ref mut view_data = *self.0.borrow_mut();
        let queue = &mut view_data.render_queue;

        view_data.shader.release(queue);

        window.start_render().unwrap();
        window.process_queue(queue).unwrap();
        window.end_render().unwrap();
    }

    fn on_changed(&mut self, window: &mut Window) {
        println!("on_changed: {:?}", window.get_size());
    }
}

impl InputEventHandler for ViewDataWrapper {
    fn on_key(&mut self, window: &mut Window, sc: ScanCode, vk: Option<VirtualKeyCode>, is_down: bool) {
        println!("key: {}, {:?}, {}", sc, vk, is_down);
    }
}


fn main() {
    let mut engine = render::Engine::new().expect("Could not initialize render engine");

    let data = Rc::new(RefCell::new(Data::new()));

    let mut window = WindowSettings::new()
        .title("main")
        .size((1024u32, 1024u32))
        .build(&mut engine).expect("Could not initialize main window");
    let window_data = ViewDataWrapper(Rc::new(RefCell::new(ViewData::new(data.clone()))));
    window.set_surface_handler(window_data.clone());
    window.set_input_handler(window_data.clone());


    let mut sub_window = WindowSettings::new()
        .title("sub")
        .size((100u32, 100u32))
        .build(&mut engine).expect("Could not initialize main window");
    let sub_window_data = ViewDataWrapper(Rc::new(RefCell::new(ViewData::new(data.clone()))));
    sub_window.set_surface_handler(sub_window_data.clone());
    sub_window.set_input_handler(sub_window_data.clone());


    let mut t = 0.;
    loop {
        if !engine.dispatch_event(DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }
        t = t + 0.01;
        if t > 1. {
            engine.quit();
            t = 0.;
        }

        if !window.is_closed() {
            window.start_render().unwrap();
            unsafe {
                gl::ClearColor(0.2, t, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            let ref mut view_data = *window_data.0.borrow_mut();
            view_data.update();
            window.process_queue(&mut view_data.render_queue).unwrap();
            window.end_render().unwrap();
        }

        if !sub_window.is_closed() {
            sub_window.start_render().unwrap();
            unsafe {
                gl::ClearColor(t, 0.2, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            let ref mut view_data = *sub_window_data.0.borrow_mut();
            view_data.update();
            sub_window.process_queue(&mut view_data.render_queue).unwrap();
            sub_window.end_render().unwrap();
        }
    }
}

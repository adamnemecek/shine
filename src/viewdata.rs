use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use world::*;
use dragorust_engine::render::*;

#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Passes {
    Present,
}

/// Structure to store view dependent data
pub struct ViewData {
    //world: WorldWrapper,
    pub render_queue: CommandQueue,
    pass_manager: PassManager<Passes>,
    shader: ShaderProgram,
    vertex_buffer: VertexBuffer,

    t: f32,
}

impl ViewData {
    fn new(_: WorldWrapper) -> ViewData {
        ViewData {
            //            world: world,
            pass_manager: PassManager::new(),
            render_queue: CommandQueue::new(),
            shader: ShaderProgram::new(),
            vertex_buffer: VertexBuffer::new(),
            t: 0f32,
        }
    }

    fn init(&mut self) -> &mut CommandQueue {
        let sh_source = [
            (ShaderType::VertexShader,
             r#"
attribute vec3 vPosition;
void main()
{
    gl_Position = vec4(vPosition, 1.0);
}
"#),
            (ShaderType::FragmentShader,
             r#"
void main()
{
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}"#)];

        let vertices = [
            VxPos { position: f32x3!(1f32, 0f32, 0f32) },
            VxPos { position: f32x3!(1f32, 1f32, 0f32) },
            VxPos { position: f32x3!(0f32, 1f32, 0f32) }
        ];

        self.shader.set_sources(&mut self.render_queue, sh_source.iter());
        self.vertex_buffer.set_transient(&mut self.render_queue, &vertices);
        self.vertex_buffer.set_transient(&mut self.render_queue, &vertices.to_vec());
        self.vertex_buffer.set_transient(&mut self.render_queue, &vertices.as_ref());
        &mut self.render_queue
    }

    fn release(&mut self) -> &mut CommandQueue {
        self.vertex_buffer.release(&mut self.render_queue);
        self.shader.release(&mut self.render_queue);
        &mut self.render_queue
    }

    fn update(&mut self) {
        self.t += 0.1f32;
        if self.t > 1f32 {
            self.t = 0f32;
        }
    }

    pub fn render(&mut self, window: &Window) -> &mut CommandQueue {
        {
            let mut pp = self.pass_manager.get(Passes::Present);

            pp.set_viewport(window.get_size());
            pp.clear(self.t);
        }

        self.pass_manager.sort_passes();
        &mut self.render_queue
    }
}


#[derive(Clone)]
pub struct ViewDataWrapper(Rc<RefCell<ViewData>>);

impl ViewDataWrapper {
    pub fn new(world: WorldWrapper) -> ViewDataWrapper {
        ViewDataWrapper(Rc::new(RefCell::new(ViewData::new(world))))
    }

    fn init(&mut self, window: &mut Window) {
        let mut view_data = self.0.borrow_mut();
        window.render_single_queue(view_data.init()).unwrap();
    }

    fn release(&mut self, window: &mut Window) {
        let mut view_data = self.0.borrow_mut();
        window.render_single_queue(view_data.release()).unwrap();
    }

    pub fn update(&self) {
        self.0.borrow_mut().update();
    }

    pub fn render(&self, window: &mut Window) {
        let mut view_data = self.0.borrow_mut();
        window.process_queue(view_data.render(window)).unwrap();
    }
}

impl Deref for ViewDataWrapper {
    type Target = RefCell<ViewData>;

    fn deref(&self) -> &RefCell<ViewData> {
        self.0.deref()
    }
}

impl SurfaceEventHandler for ViewDataWrapper {
    fn on_ready(&mut self, window: &mut Window) {
        println!("on_ready");
        self.init(window);
    }

    fn on_lost(&mut self, window: &mut Window) {
        println!("on_lost");
        self.release(window);
    }

    fn on_changed(&mut self, window: &mut Window) {
        println!("on_changed: {:?}", window.get_size());
    }
}

impl InputEventHandler for ViewDataWrapper {
    fn on_key(&mut self, _: &mut Window, sc: ScanCode, vk: Option<VirtualKeyCode>, is_down: bool) {
        println!("key: {}, {:?}, {}", sc, vk, is_down);
    }
}
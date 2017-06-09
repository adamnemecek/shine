use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use world::*;
use dragorust_engine::render::*;

#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

/// Structure to store view dependent data
pub struct ViewData {
    world: WorldWrapper,
    pub render_queue: CommandQueue,
    shader: ShaderProgram,
    vertex_buffer: VertexBuffer,
}

impl ViewData {
    fn new(world: WorldWrapper) -> ViewData {
        ViewData {
            world: world,
            render_queue: CommandQueue::new(),
            shader: ShaderProgram::new(),
            vertex_buffer: VertexBuffer::new(),
        }
    }

    pub fn update(&mut self) {}
}


#[derive(Clone)]
pub struct ViewDataWrapper(Rc<RefCell<ViewData>>);

impl ViewDataWrapper {
    pub fn new(world: WorldWrapper) -> ViewDataWrapper {
        ViewDataWrapper(Rc::new(RefCell::new(ViewData::new(world))))
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
        let ref mut view_data = *self.0.borrow_mut();
        let queue = &mut view_data.render_queue;

        let sh_source = [
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

        let vertices = [VxPos { position: f32x3!(1f32, 0f32, 0f32) }, VxPos { position: f32x3!(1f32, 1f32, 0f32) }, VxPos { position: f32x3!(0f32, 1f32, 0f32) }].to_vec();

        view_data.shader.set_sources(queue, sh_source.iter());
        view_data.vertex_buffer.set_transient(queue, &vertices );
        //VxPos::get_declaration(vertices.len()).iter(), unsafe { slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * mem::size_of::<VxPos>()) }

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
    fn on_key(&mut self, _: &mut Window, sc: ScanCode, vk: Option<VirtualKeyCode>, is_down: bool) {
        println!("key: {}, {:?}, {}", sc, vk, is_down);
    }
}
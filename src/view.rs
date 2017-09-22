use std::rc::Rc;
use std::cell::RefCell;
use std::mem::transmute;

use world::*;
use dragorust_engine::render::*;

#[derive(Copy, Clone, Debug)]
#[derive(VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

#[derive(Copy, Clone, Debug)]
#[derive(VertexDeclaration)]
struct VxColorTex {
    color: Float32x3,
    tex_coord: Float32x2,
}


#[derive(Copy, Clone, Debug)]
//#[derive(ShaderDeclaration)]
struct ShSimple {}

impl ShaderDeclaration for ShSimple {
    type Attribute = ShSimpleAttribute;
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
#[allow(non_camel_case_types)]
enum ShSimpleAttribute {
    vPosition,
    vColor,
    vTexCoord,
}

impl PrimitiveEnum for ShSimpleAttribute {
    fn from_index_unsafe(index: usize) -> ShSimpleAttribute {
        assert!(index < Self::count(), "invalid index {}, must be in the range 0..{}", index, Self::count());
        unsafe { transmute(index) }
    }

    fn from_name(name: &str) -> Option<ShSimpleAttribute> {
        match name {
            "vPosition" => Some(ShSimpleAttribute::vPosition),
            "vColor" => Some(ShSimpleAttribute::vColor),
            "vTexCoord" => Some(ShSimpleAttribute::vTexCoord),
            _ => None
        }
    }

    fn to_index(&self) -> usize {
        unsafe { transmute(*self) }
    }

    fn count() -> usize {
        3
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Passes {
    Present,
    //Shadow,
    //Debug,
}

impl PassKey for Passes {}


/// Structure to handle view dependent data
pub struct ViewData {
    world: World,
    render: RenderManager<Passes>,

    shader: ShaderProgram<ShSimple>,

    vertex_buffer1: VertexBuffer<VxPos>,
    vertex_buffer2: VertexBuffer<VxColorTex>,

    t: f32,
}

impl ViewData {
    fn new(world: World) -> ViewData {
        ViewData {
            world: world,
            render: RenderManager::new(),
            shader: ShaderProgram::new(),
            vertex_buffer1: VertexBuffer::new(),
            vertex_buffer2: VertexBuffer::new(),
            t: 0f32,
        }
    }

    fn init(&mut self, window: &Window) {
        // create some shaders
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

        // create some geometry
        let pos = [
            VxPos { position: f32x3!(1f32, 0f32, 0f32) },
            VxPos { position: f32x3!(1f32, 1f32, 0f32) },
            VxPos { position: f32x3!(0f32, 1f32, 0f32) }
        ];

        // create some geometry
        let color_tex = [
            VxColorTex { color: f32x3!(1f32, 0f32, 0f32), tex_coord: f32x2!(1, 0) },
            VxColorTex { color: f32x3!(1f32, 1f32, 0f32), tex_coord: f32x2!(1, 1) },
            VxColorTex { color: f32x3!(0f32, 1f32, 0f32), tex_coord: f32x2!(0, 0) }
        ];

        // upload data
        self.shader.set_sources(&mut self.render, sh_source.iter());
        self.vertex_buffer1.set_transient(&mut self.render, &pos);
        self.vertex_buffer2.set_transient(&mut self.render, &color_tex.to_vec());

        // submit commands
        self.render.submit(window);
    }

    fn release(&mut self, window: &Window) {
        self.vertex_buffer1.release(&mut self.render);
        self.vertex_buffer2.release(&mut self.render);
        self.shader.release(&mut self.render);
        self.render.submit(window);
    }

    fn update(&mut self) {
        self.t += 0.005f32;
        if self.t > 1f32 {
            self.t = 0f32;
        }
    }

    pub fn render(&mut self, window: &Window) {
        {
            let mut p0 = self.render.get_pass(Passes::Present);
            p0.config_mut().set_clear_color(f32x3!(self.t, self.world.get_t(), 0.));
            p0.config_mut().set_fullscreen();

            let v1 = &self.vertex_buffer1;
            let v2 = &self.vertex_buffer2;

            self.shader.draw(&mut *p0,
                             |id| match id {
                                 ShSimpleAttribute::vPosition => v1.get_attribute(VxPosAttribute::Position),
                                 ShSimpleAttribute::vColor => v2.get_attribute(VxColorTexAttribute::Color),
                                 ShSimpleAttribute::vTexCoord => v2.get_attribute(VxColorTexAttribute::TexCoord),
                             },
                             Primitive::Triangle, 0, 3
            );
        }

        {
            //let mut p1 = self.render.get_pass(Passes::Shadow);
        }

        self.render.submit(window);
    }
}


/// Structure to wrap ViewData for surface event handling.
pub struct ViewSurfaceHandler(Rc<RefCell<ViewData>>);

impl SurfaceEventHandler for ViewSurfaceHandler {
    fn on_ready(&mut self, window: &Window) {
        self.0.borrow_mut().init(window);
    }

    fn on_lost(&mut self, window: &Window) {
        self.0.borrow_mut().release(window);
    }

    fn on_changed(&mut self, _window: &Window) {
        //todo: println!("on_changed: {:?}", window.get_size());
    }

    fn on_render(&mut self, window: &Window) {
        self.0.borrow_mut().render(window);
    }
}


/// Structure to wrap ViewData for input event handling.
pub struct ViewInputHandler(Rc<RefCell<ViewData>>);

impl InputEventHandler for ViewInputHandler {
    fn on_key(&mut self, _: &mut Window, sc: ScanCode, vk: Option<VirtualKeyCode>, is_down: bool) {
        println!("key: {}, {:?}, {}", sc, vk, is_down);
    }
}


/// Structure to wrap ViewData for public use.
pub struct View(Rc<RefCell<ViewData>>);

impl View {
    pub fn new(world: World, window: &mut Window) -> View {
        let view = View(Rc::new(RefCell::new(ViewData::new(world))));
        window.set_surface_handler(ViewSurfaceHandler(view.0.clone()));
        window.set_input_handler(ViewInputHandler(view.0.clone()));
        view
    }

    pub fn update(&self) {
        self.0.borrow_mut().update();
    }
}

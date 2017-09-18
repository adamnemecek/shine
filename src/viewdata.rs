use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
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
    tex: Float32x2,
}


/*shader_enum!( SimpleShaderAttribute {
    Position,
    Color,
    TexCoord,

    Count,
})*/

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
enum SimpleShaderAttribute {
    Position,
    Color,
    TexCoord,

    Count,
}

impl ShaderAttributeEnum for SimpleShaderAttribute {
    fn from_index(index: usize) -> Option<SimpleShaderAttribute> {
        if index < Self::count() {
            Some(unsafe { transmute(index) })
        } else {
            None
        }
    }

    fn from_name(name: &str) -> Option<SimpleShaderAttribute> {
        match name {
            "position" => Some(SimpleShaderAttribute::Position),
            "color" => Some(SimpleShaderAttribute::Color),
            "texCoord" => Some(SimpleShaderAttribute::TexCoord),
            _ => None
        }
    }

    fn to_index(&self) -> usize {
        unsafe { transmute(*self) }
    }

    fn count() -> usize {
        SimpleShaderAttribute::Count.to_index()
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Passes {
    Present,
    Shadow,
    //Debug,
}

impl PassKey for Passes {}


/// Structure to store view dependent data
pub struct ViewData {
    render: RenderManager<Passes>,

    shader: ShaderProgram<SimpleShaderAttribute>,

    vertex_buffer1: VertexBuffer,
    vertex_buffer2: VertexBuffer,

    t: f32,
}

impl ViewData {
    fn new(_: WorldWrapper) -> ViewData {
        ViewData {
            //            world: world,
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
        let vertices = [
            VxPos { position: f32x3!(1f32, 0f32, 0f32) },
            VxPos { position: f32x3!(1f32, 1f32, 0f32) },
            VxPos { position: f32x3!(0f32, 1f32, 0f32) }
        ];

        // create some geometry
        let colorTex = [
            VxColorTex { color: f32x3!(1f32, 0f32, 0f32), tex: f32x2!(1, 0) },
            VxColorTex { color: f32x3!(1f32, 1f32, 0f32), tex: f32x2!(1, 1) },
            VxColorTex { color: f32x3!(0f32, 1f32, 0f32), tex: f32x2!(0, 0) }
        ];

        // upload data
        self.shader.set_sources(&mut self.render, sh_source.iter());
        self.vertex_buffer1.set_transient(&mut self.render, &vertices);
        self.vertex_buffer2.set_transient(&mut self.render, &colorTex.to_vec());

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
        self.t += 0.001f32;
        if self.t > 1f32 {
            self.t = 0f32;
        }
    }

    pub fn render(&mut self, window: &Window) {
        {
            let mut p0 = self.render.get_pass(Passes::Present);
            p0.config_mut().set_clear_color(f32x3!(self.t, 0, 0));
            p0.config_mut().set_fullscreen();

            //self.shader.draw::<SimpleShaderAttribute, RenderPass>(&mut *p0);

            /*shaderprogram.draw(
                p0,
                |id| {
                    match id {
                        Position => self.vertex_buffer1.attribute(VxPosLocation::Position),
                        Color => self.vertex_buffer2.attribute(VxColorTexLocation::Color),
                        Color => self.vertex_buffer2.attribute(VxColorTexLocation::Texture),
                    }
                },
                Primitive::Triangle, 0, 3);*/
        }

        {
            let mut p1 = self.render.get_pass(Passes::Shadow);
        }

        self.render.submit(window);
    }
}


#[derive(Clone)]
pub struct ViewDataWrapper(Rc<RefCell<ViewData>>);

impl ViewDataWrapper {
    pub fn new(world: WorldWrapper) -> ViewDataWrapper {
        ViewDataWrapper(Rc::new(RefCell::new(ViewData::new(world))))
    }

    fn init(&mut self, window: &mut Window) {
        self.0.borrow_mut().init(window);
    }

    fn release(&mut self, window: &mut Window) {
        self.0.borrow_mut().release(window);
    }

    pub fn update(&self) {
        self.0.borrow_mut().update();
    }

    pub fn render(&self, window: &mut Window) {
        self.0.borrow_mut().render(window);
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
use std::rc::*;
use std::cell::*;
use std::slice;
use std::str;
use std::f32;

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

#[derive(ShaderDeclaration)]
#[vert_src = "
    uniform mat4 uTrsf;
    uniform vec3 uColor;
    attribute vec3 vPosition;
    attribute vec3 vColor;
    varying vec3 color;
    void main()
    {
        color = vColor; + uColor;
        gl_Position = uTrsf * vec4(vPosition, 1.0);
    }
"]
#[frag_src = "
    varying vec3 color;
    void main()
    {
        gl_FragColor = vec4(color, 1.0);
    }
"]
//todo 1:
//#[state(depth) = on]
//#[state(clamp) = ccw]
//todo 2:
//#[state(point_size) = ?]
//todo 3:
//#[unifrom(uTrsf) = engine.trsf]
struct ShSimple {}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Passes {
    Present,
    //Shadow,
    //Debug,
}

impl PassKey for Passes {}


/// Structure to handle view dependent data
pub struct SimpleView {
    world: Rc<RefCell<World>>,
    render: RenderManager<Passes>,

    shader: ShaderProgram<ShSimple>,

    vertex_buffer1: VertexBuffer<VxPos>,
    vertex_buffer2: VertexBuffer<VxColorTex>,
    index_buffer1: IndexBuffer<u8>,
    index_buffer2: IndexBuffer<u16>,
    texture1: Texture2D,
    texture2: Texture2D,

    t: f32,
}

impl SimpleView {
    pub fn new(world: Rc<RefCell<World>>) -> SimpleView {
        SimpleView {
            world: world,
            render: RenderManager::new(),
            shader: ShaderProgram::new(),
            vertex_buffer1: VertexBuffer::new(),
            vertex_buffer2: VertexBuffer::new(),
            index_buffer1: IndexBuffer::new(),
            index_buffer2: IndexBuffer::new(),
            texture1: Texture2D::new(),
            texture2: Texture2D::new(),
            t: 0f32,
        }
    }
}

impl View for SimpleView {
    fn on_surface_ready(&mut self, window: &mut Window) {
        // create some shaders


        // create some geometry
        let pos = [
            VxPos { position: f32x3!(1, 0, 0) },
            VxPos { position: f32x3!(1, 1, 0) },
            VxPos { position: f32x3!(0, 1, 0) },
            VxPos { position: f32x3!(0, 0, 0) },
        ];

        // create some geometry
        let color_tex = [
            VxColorTex { color: f32x3!(1, 0, 0), tex_coord: f32x2!(1, 0) },
            VxColorTex { color: f32x3!(1, 1, 0), tex_coord: f32x2!(1, 1) },
            VxColorTex { color: f32x3!(0, 1, 0), tex_coord: f32x2!(0, 0) },
            VxColorTex { color: f32x3!(0, 0, 0), tex_coord: f32x2!(0, 0) }
        ];

        let index1 = [0u8, 1, 2, 0, 2, 3];
        let index2 = [0u16, 1, 2, 0, 2, 3];

        // upload data
        //self.shader.set_sources(&mut self.render, sh_source.iter());
        self.shader.compile(&mut self.render);
        self.vertex_buffer1.set_transient(&mut self.render, &pos);
        self.vertex_buffer2.set_transient(&mut self.render, &color_tex.to_vec());

        self.index_buffer1.set_transient(&mut self.render, &index1);
        // self.index_buffer1.set_transient(&mut self.render, &index2);  // shall not compile
        self.index_buffer2.set_transient(&mut self.render, &index2.to_vec());

        // submit commands
        self.render.submit(window);
    }

    fn on_surface_lost(&mut self, window: &mut Window) {
        self.vertex_buffer1.release(&mut self.render);
        self.vertex_buffer2.release(&mut self.render);
        self.index_buffer1.release(&mut self.render);
        self.index_buffer2.release(&mut self.render);
        self.texture1.release(&mut self.render);
        self.texture2.release(&mut self.render);
        self.shader.release(&mut self.render);
        self.render.submit(window);
    }

    fn on_surface_changed(&mut self, _window: &mut Window) {
        // nop
    }

    fn on_update(&mut self) {
        self.t += 0.05f32;
        if self.t > 2. * f32::consts::PI {
            self.t = 0f32;
        }
    }

    fn on_render(&mut self, window: &mut Window) {
        {
            let mut p0 = self.render.get_pass(Passes::Present);
            p0.config_mut().set_clear_color(f32x3!(self.t, self.world.borrow().get_t(), 0.));
            p0.config_mut().set_fullscreen();

            let v1 = &self.vertex_buffer1;
            let v2 = &self.vertex_buffer2;

            let attributes = ShSimpleAttribute {
                position: v1.get_attribute_ref(VxPosAttribute::Position),
                color: v2.get_attribute_ref(VxColorTexAttribute::Color),
                //tex_coord: v2.get_attribute_ref(VxColorTexAttribute::TexCoord),
            };

            let st = self.t.sin();
            let ct = self.t.cos();

            {
                let uniforms = ShSimpleUniform {
                    trsf: f32x16!(st, -ct, 0, 0,
                                   ct,  st, 0, 0,
                                    0,   0, 1, 0,
                                    0,   0, 0, 1),
                    color: f32x3!(1.2f32, 0.2f32, 0.2f32),
                    //tex: texture1.get_ref(),
                };
                self.shader.draw(&mut *p0, attributes.clone(), uniforms.clone(), Primitive::Triangle, 0, 3);
            }

            {
                let uniforms = ShSimpleUniform {
                    trsf: f32x16!(ct, -st, 0, 0,
                                  st,  ct, 0, 0,
                                   0,   0, 1, 0,
                                   0,   0, 0, 1),
                    color: f32x3!(1.2f32, 0.2f32, 0.2f32),
                    //tex: texture1.get_ref(),
                };
                self.shader.draw_indexed(&mut *p0, attributes.clone(), self.index_buffer1.get_ref(), uniforms, Primitive::Triangle, 0, 6);
            }
        }

        {
            //let mut p1 = self.render.get_pass(Passes::Shadow);
        }

        self.render.submit(window);
    }

    fn on_key(&mut self, window: &mut Window, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { window.close(); }
            _ => {}
        }
    }
}

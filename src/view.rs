use std::slice;
use std::str;
use std::f32;

use game;
use world;
use render::*;

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
    attribute vec2 vTexCoord;
    varying vec3 color;
    varying vec2 txCoord;
    void main()
    {
        color = vColor; + uColor;
        txCoord = vTexCoord;
        gl_Position = uTrsf * vec4(vPosition, 1.0);
    }
"]
#[frag_src = "
    varying vec3 color;
    varying vec2 txCoord;
    uniform sampler2D uTex;
    void main()
    {
        vec3 txColor = texture2D( uTex, txCoord ).rgb;
        vec3 col = txColor * color;
        gl_FragColor = vec4(col, 1.0);
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
    game: game::GameCell,
    render: RenderManager<Passes>,

    shader: ShaderProgram<ShSimple>,

    vertex_buffer1: VertexBufferHandle<VxPos>,
    vertex_buffer2: VertexBufferHandle<VxColorTex>,
    index_buffer1: IndexBufferHandle<u8>,
    index_buffer2: IndexBufferHandle<u16>,
    img1: world::ImageRef,
    texture1: Texture2DHandle,
    texture2: Texture2DHandle,

    t: f32,
}

impl SimpleView {
    pub fn new(game: game::GameCell) -> SimpleView {
        //let mut render = RenderManager::new();

        let mut view = SimpleView {
            game: game,
            render: RenderManager::new(),

            shader: ShaderProgram::new(),
            vertex_buffer1: VertexBufferHandle::null(),
            vertex_buffer2: VertexBufferHandle::null(),
            index_buffer1: IndexBufferHandle::null(),
            index_buffer2: IndexBufferHandle::null(),
            texture1: Texture2DHandle::null(),
            texture2: Texture2DHandle::null(),

            img1: world::ImageRef::null(),
            t: 0f32,
        };

        view.vertex_buffer1 = VertexBufferHandle::create(&mut view.render);
        view.vertex_buffer2 = VertexBufferHandle::create(&mut view.render);
        view.index_buffer1 = IndexBufferHandle::create(&mut view.render);
        view.index_buffer2 = IndexBufferHandle::create(&mut view.render);
        view.texture1 = Texture2DHandle::create(&mut view.render);

        view
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
            VxColorTex { color: f32x3!(0, 1, 0), tex_coord: f32x2!(0, 1) },
            VxColorTex { color: f32x3!(0, 0, 0), tex_coord: f32x2!(0, 0) }
        ];

        let index1 = [0u8, 1, 2, 0, 2, 3];
        let index2 = [0u16, 1, 2, 0, 2, 3];

        // upload data
        //self.shader.set_sources(&mut self.render, sh_source.iter());
        self.shader.compile(&mut self.render);
        self.vertex_buffer1.set(&mut self.render, &pos);
        self.vertex_buffer2.set(&mut self.render, &color_tex.to_vec());

        self.index_buffer1.set(&mut self.render, &index1);
        // self.index_buffer1.set_transient(&mut self.render, &index2);  // shall not compile
        self.index_buffer2.set(&mut self.render, &index2.to_vec());

        // submit commands
        self.render.submit(window);
    }

    fn on_surface_lost(&mut self, window: &mut Window) {
        self.vertex_buffer1.reset();
        self.vertex_buffer2.reset();
        self.index_buffer1.reset();
        self.index_buffer2.reset();
        self.texture1.reset();
        self.texture2.reset();
        //self.shader.reset();
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
        let game = self.game.borrow();

        {
            let image_store = game.image_store.read();

            if self.img1.is_null() {
                self.img1 = image_store.get_or_request(&world::ImageId::new("c:\\dev\\alma.png"));
            }
            self.texture1.set(&mut self.render, image_store[&self.img1].get_image());
        }

        {
            let mut p0 = self.render.get_pass(Passes::Present);
            p0.config_mut().set_clear_color(f32x3!(self.t, game.t, 0.));
            p0.config_mut().set_fullscreen();

            let st = self.t.sin();
            let ct = self.t.cos();

            let parameters = ShSimpleParameters {
                v_position: From::from((&self.vertex_buffer1, VxPosAttribute::Position)),
                v_color: From::from((&self.vertex_buffer2, VxColorTexAttribute::Color)),
                v_tex_coord: From::from((&self.vertex_buffer2, VxColorTexAttribute::TexCoord)),

                indices: Default::default(),

                u_trsf: f32x16!(st, -ct, 0, 0,
                                    ct,  st, 0, 0,
                                     0,   0, 1, 0,
                                     0,   0, 0, 1),
                u_color: f32x3!(1.2f32, 0.2f32, 0.2f32),
                u_tex: From::from(&self.texture1),
            };

            {
                let parameters = {
                    let mut p = parameters.clone();
                    p.u_trsf = f32x16!(st, -ct, 0, 0,
                                       ct,  st, 0, 0,
                                       0,    0, 1, 0,
                                       0,    0, 0, 1);
                    p.u_color = f32x3!(1.2f32, 0.2f32, 0.2f32);
                    p.u_tex = From::from(&self.texture1);
                    p
                };

                self.shader.draw(&mut *p0, parameters, Primitive::Triangle, 0, 3);
            }

            {
                let parameters = {
                    let mut p = parameters.clone();
                    p.indices = From::from(&self.index_buffer1);
                    p.u_trsf = f32x16!(ct, -st, 0, 0,
                                       st,  ct, 0, 0,
                                       0,    0, 1, 0,
                                       0,    0, 0, 1);
                    p.u_color = f32x3!(1.2f32, 0.2f32, 0.2f32);
                    p.u_tex = From::from(&self.texture1);
                    p
                };

                self.shader.draw(&mut *p0, parameters, Primitive::Triangle, 0, 6);
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

extern crate image;
#[macro_use]
extern crate shine_render as render;

use std::env;
use render::*;

#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxColorTex {
    color: Float32x3,
    tex_coord: Float32x2,
}

#[derive(Copy, Clone, Debug, GLShaderDeclaration)]
#[vert_path = "fun.glsl"]
#[vert_src = "
    attribute vec3 vPosition;
    attribute vec3 vColor;
    attribute vec2 vTexCoord;
    uniform mat4 uTrsf;
    uniform vec3 uColor;
    varying vec3 color;
    varying vec2 txCoord;
    void main()
    {
        color = uColor * vColor;
        txCoord = vTexCoord.xy;
        gl_Position = uTrsf * vec4(vPosition, 1.0);
    }"]
#[frag_src = "
    varying vec3 color;
    varying vec2 txCoord;
    uniform sampler2D uTex;
    void main()
    {
        float intensity = texture2D( uTex, txCoord ).r;
        vec3 col =  color * intensity;
        gl_FragColor = vec4(col, 1.0);
    }"]
#[depth = "?"]
#[cull = "cw"]
struct ShSimple {}

struct SimpleView {
    t: f32,
    vb1: VertexBufferHandle<VxPos>,
    vb2: VertexBufferHandle<VxColorTex>,
    ib: IndexBufferHandle<u8>,
    tx: Texture2DHandle,
    sh: ShaderProgramHandle<ShSimple>,
}

impl SimpleView {
    fn new() -> SimpleView {
        SimpleView {
            t: 0.0,
            vb1: Handle::null(),
            vb2: Handle::null(),
            ib: Handle::null(),
            tx: Handle::null(),
            sh: Handle::null(),
        }
    }
}

impl View<PlatformEngine> for SimpleView {
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        println!("surface ready");
        let mut queue = r.get_queue();

        let pos = [
            VxPos { position: (1., 0., 0.).into() },
            VxPos { position: (1., 1., 0.).into() },
            VxPos { position: (0., 1., 0.).into() },
            VxPos { position: (0., 0., 0.).into() },
        ];
        self.vb1.create_and_set(&mut queue, &pos);

        let color_tex = [
            VxColorTex { color: (1., 0., 0.).into(), tex_coord: (1., 0.).into() },
            VxColorTex { color: (1., 1., 0.).into(), tex_coord: (1., 1.).into() },
            VxColorTex { color: (0., 1., 0.).into(), tex_coord: (0., 1.).into() },
            VxColorTex { color: (0., 0., 0.).into(), tex_coord: (0., 0.).into() },
        ];
        self.vb2.create_and_set(&mut queue, &color_tex);

        let indices = [0u8, 1, 2, 0, 2, 3];
        self.ib.create_and_set(&mut queue, &indices);

        let img = include_bytes!("img.jpg");
        let img = image::load_from_memory(img).unwrap();
        self.tx.create_and_set(&mut queue, &img);

        self.sh.create_and_compile(&mut queue);
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        println!("surface lost");
        self.vb1.reset();
        self.vb2.reset();
        self.ib.reset();
        self.tx.reset();
        self.sh.reset();
    }

    fn on_surface_changed(&mut self, ctl: &mut WindowControl, r: &mut PlatformBackend) {
        println!("surface changed");
        self.on_surface_lost(ctl, r);
        self.on_surface_ready(ctl, r);
    }

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        use std::f32;
        self.t += 0.05f32;
        if self.t > 2. * f32::consts::PI {
            self.t = 0f32;
        }
    }

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        let mut queue = r.get_queue();

        r.init_view(Some(Viewport::Proportional(0.5, 0.5, 0.25, 0.25)),
                    Some(Float32x4(0., 0., 0., 1.)),
                    None);

        let st = self.t.sin();
        let ct = self.t.cos();

        let params = ShSimpleParameters {
            v_color: (&self.vb2, VxColorTex::COLOR).into(),
            v_tex_coord: (&self.vb2, VxColorTex::TEXCOORD).into(),
            v_position: (&self.vb1, VxPos::POSITION).into(),
            indices: (&self.ib).into(),
            u_color: Float32x3::from([0.5, self.t / 6.28, 0.5]),
            u_trsf: Float32x16::from([
                st, -ct, 0.0, 0.0, ct, st, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            ]),
            u_tex: (&self.tx).into(),
            depth: DepthFunction::Disable,
        };

        self.sh.draw(&mut queue, params, Primitive::Triangles, 0, 6);
    }

    fn on_key(
        &mut self,
        ctl: &mut WindowControl,
        _scan_code: ScanCode,
        virtual_key: Option<VirtualKeyCode>,
        is_down: bool,
    ) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => {
                ctl.close();
            }
            _ => {}
        }
    }
}

#[test]
pub fn render() {
    assert!(
        env::var("RUST_TEST_THREADS").unwrap_or("0".to_string()) == "1",
        "This test shall run in single threaded test environment: RUST_TEST_THREADS=1"
    );

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((1024, 1024))
        .build(&engine, SimpleView::new())
        .expect("Could not initialize main window");


    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((256, 256))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
        .build(&engine, SimpleView::new()).expect("Could not initialize sub window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Immediate) {
            break;
        }

        window.update_view();
        sub_window.update_view();

        window.render().unwrap();
        sub_window.render().unwrap();
    }
}

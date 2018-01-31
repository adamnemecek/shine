#[macro_use]
extern crate dragorust_render_gl as render;
extern crate image;

use std::time::Duration;
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

#[derive(Copy, Clone, Debug)]
#[derive(GLShaderDeclaration)]
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
//todo 1:
//#[state(depth) = on]
//#[state(clamp) = ccw]
//todo 2:
//#[state(point_size) = ?]
//todo 3:
//#[unifrom(uTrsf) = engine.trsf]
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
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut GLBackend) {
        println!("surface ready");
        let mut compose = r.compose();

        let pos = [
            VxPos { position: f32x3!(1, 0, 0) },
            VxPos { position: f32x3!(1, 1, 0) },
            VxPos { position: f32x3!(0, 1, 0) },
            VxPos { position: f32x3!(0, 0, 0) },
        ];
        self.vb1.create_and_set(&mut compose, &pos);

        let color_tex = [
            VxColorTex { color: f32x3!(1, 0, 0), tex_coord: f32x2!(1, 0) },
            VxColorTex { color: f32x3!(1, 1, 0), tex_coord: f32x2!(1, 1) },
            VxColorTex { color: f32x3!(0, 1, 0), tex_coord: f32x2!(0, 1) },
            VxColorTex { color: f32x3!(0, 0, 0), tex_coord: f32x2!(0, 0) }
        ];
        self.vb2.create_and_set(&mut compose, &color_tex);

        let indices = [0u8, 1, 2, 0, 2, 3];
        self.ib.create_and_set(&mut compose, &indices);

        let img = include_bytes!("img.jpg");
        let img = image::load_from_memory(img).unwrap();
        self.tx.create_and_set(&mut compose, &img);

        self.sh.create_and_compile(&mut compose);
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {
        println!("surface lost");
        self.vb1.reset();
        self.vb2.reset();
        self.ib.reset();
        self.tx.reset();
        self.sh.reset();
    }

    fn on_surface_changed(&mut self, ctl: &mut WindowControl, r: &mut GLBackend) {
        println!("surface changed");
        self.on_surface_lost(ctl, r);
        self.on_surface_ready(ctl, r);
    }

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {
        use std::f32;
        self.t += 0.05f32;
        if self.t > 2. * f32::consts::PI {
            self.t = 0f32;
        }
    }

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut GLBackend) {
        let mut compose = r.compose();


        {
            use render::lowlevel::*;

            let ll = r.ll_mut();

            ugl!(ClearColor(0.0, 0.0, 0.5, 1.0));
            ugl!(Clear(gl::COLOR_BUFFER_BIT));
            ll.states.set_viewport(lowlevel::Viewport::Proportional(0.5, 0.5, 0.25, 0.25));
        }

        let params = ShSimpleParameters {};
        /*let st = self.t.sin();
        let ct = self.t.cos();
        let trsf = Float32x16::from(
            [st, -ct, 0.0, 0.0,
                ct, st, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0]);
        let col = Float32x3::from([0.5, self.t / 6.28, 0.5]);
        */

        self.sh.draw(&mut compose, params, Primitive::Triangle, 0, 6);
    }

    fn on_key(&mut self, ctl: &mut WindowControl, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { ctl.close(); }
            _ => {}
        }
    }
}

#[test]
pub fn render() {
    assert!(env!("RUST_TEST_THREADS") == "1", "This test shall run in single threaded test environment: RUST_TEST_THREADS=1");

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((1024, 1024))
        .build(&engine, SimpleView::new()).expect("Could not initialize main window");

    /*
    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((256, 256))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
        .build(&engine, SimpleView::new()).expect("Could not initialize sub window");
*/
    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        window.update_view();
        //sub_window.update_view();

        window.render().unwrap();
//        sub_window.render().unwrap();
    }
}

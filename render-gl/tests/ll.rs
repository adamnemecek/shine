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
        color = col_mod(uColor * vColor);
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
    vb1: lowlevel::GLVertexBuffer,
    vb2: lowlevel::GLVertexBuffer,
    ib: lowlevel::GLIndexBuffer,
    sh: lowlevel::GLShaderProgram,
    tx: lowlevel::GLTexture,
}

impl SimpleView {
    fn new() -> SimpleView {
        SimpleView {
            t: 0.0,
            vb1: lowlevel::GLVertexBuffer::new(),
            vb2: lowlevel::GLVertexBuffer::new(),
            ib: lowlevel::GLIndexBuffer::new(),
            sh: lowlevel::GLShaderProgram::new(),
            tx: lowlevel::GLTexture::new(),
        }
    }
}

impl View<PlatformEngine> for SimpleView {
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut GLBackend) {
        use lowlevel::*;

        let ll = r.ll_mut();

        {
            let pos = [
                VxPos { position: f32x3!(1, 0, 0) },
                VxPos { position: f32x3!(1, 1, 0) },
                VxPos { position: f32x3!(0, 1, 0) },
                VxPos { position: f32x3!(0, 0, 0) },
            ];

            let VertexData::Transient(slice) = pos.to_data();
            let attributes: GLVertexBufferFormat = VxPos::attribute_layout_iter()
                .map(|a| GLVertexBufferAttribute::from_layout(&a))
                .collect();
            self.vb1.upload_data(ll, attributes, slice);
        }

        {
            let color_tex = [
                VxColorTex { color: f32x3!(1, 0, 0), tex_coord: f32x2!(1, 0) },
                VxColorTex { color: f32x3!(1, 1, 0), tex_coord: f32x2!(1, 1) },
                VxColorTex { color: f32x3!(0, 1, 0), tex_coord: f32x2!(0, 1) },
                VxColorTex { color: f32x3!(0, 0, 0), tex_coord: f32x2!(0, 0) }
            ];

            let VertexData::Transient(slice) = color_tex.to_data();
            let attributes: GLVertexBufferFormat = VxColorTex::attribute_layout_iter()
                .map(|a| GLVertexBufferAttribute::from_layout(&a))
                .collect();
            self.vb2.upload_data(ll, attributes, slice);
        }

        {
            let indices = [0u8, 1, 2, 0, 2, 3];

            let IndexData::Transient(slice) = indices.to_data();
            self.ib.upload_data(ll, IndexBinding::glenum_from_index_type(<u8 as IndexType>::get_layout()), slice);
        }

        {
            let img = include_bytes!("img.jpg");
            let img = image::load_from_memory(img).unwrap();

            let ImageData::Transient(width, height, format, slice) = img.to_data();
            self.tx.upload_data(ll, gl::TEXTURE_2D, width, height, TextureBinding::glenum_from_pixel_format(format), slice);
        }

        {
            self.sh.create_program(ll, ShSimple::source_iter());
            self.sh.parse_parameters(ll,
                                     |n| match n {
                                         "vPosition" => 0,
                                         "vColor" => 1,
                                         "vTexCoord" => 2,
                                         _ => panic!("unknown attribute: {}", n)
                                     },
                                     |n| match n {
                                         "uTrsf" => 3,
                                         "uColor" => 4,
                                         "uTex" => 5,
                                         _ => panic!("unknown uniform: {}", n)
                                     });
        }
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, r: &mut GLBackend) {
        let ll = r.ll_mut();
        self.vb1.release(ll);
        self.vb2.release(ll);
        self.ib.release(ll);
        self.tx.release(ll);
        self.sh.release(ll);
    }

    fn on_surface_changed(&mut self, ctl: &mut WindowControl, r: &mut GLBackend) {
        //emulate full surface lost on window resize
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
        use render::lowlevel::*;

        let ll = r.ll_mut();

        ugl!(ClearColor(0.0, 0.0, 0.5, 1.0));
        ugl!(Clear(gl::COLOR_BUFFER_BIT));

        //ll.states.set_viewport(lowlevel::Viewport::FullScreen);
        ll.states.set_viewport(lowlevel::Viewport::Proportional(0.5, 0.5, 0.25, 0.25));

        let st = self.t.sin();
        let ct = self.t.cos();
        let trsf = Float32x16::from(
            [st, -ct, 0.0, 0.0,
                ct, st, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0]);
        let col = Float32x3::from([0.5, self.t / 6.28, 0.5]);

        let sh = &mut self.sh;
        let vb1 = &mut self.vb1;
        let vb2 = &mut self.vb2;
        let ib = &mut self.ib;
        let tx = &mut self.tx;
        sh.draw(ll, gl::TRIANGLES, 0, 6,
                |ll, locations| {
                    ib.bind(ll);
                    locations[0].set_attribute(ll, &vb1, VxPosAttribute::Position); // "vPosition" => 0
                    locations[1].set_attribute(ll, &vb2, VxColorTexAttribute::Color); //"vColor" => 1,
                    locations[2].set_attribute(ll, &vb2, VxColorTexAttribute::TexCoord); //"vTexCoord" => 2,
                    locations[3].set_f32x16(ll, &trsf); //"uTrsf" => 3
                    locations[4].set_f32x3(ll, &col); //"uColor" => 4
                    locations[5].set_texture(ll, &tx); //"uTex" => 5,
                });
    }

    fn on_key(&mut self, ctl: &mut WindowControl, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { ctl.close(); }
            _ => {}
        }
    }
}

#[test]
pub fn simple_lowlevel() {
    assert!(env!("RUST_TEST_THREADS") == "1", "This test shall run in single threaded test environment: RUST_TEST_THREADS=1");

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((1024, 1024))
        .build(&engine, SimpleView::new()).expect("Could not initialize main window");

    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((256, 256))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
        .build(&engine, SimpleView::new()).expect("Could not initialize sub window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        window.update_view();
        sub_window.update_view();

        window.render().unwrap();
        sub_window.render().unwrap();
    }
}

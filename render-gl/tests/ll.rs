#[macro_use]
extern crate dragorust_render_gl as render;

use std::time::Duration;
use render::*;

#[derive(Copy, Clone, Debug)]
#[derive(VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

struct SimpleView {
    vb: lowlevel::GLVertexBuffer,
    ib: lowlevel::GLIndexBuffer,
    sh: lowlevel::GLShaderProgram,
    tx: lowlevel::GLTexture,
}

impl SimpleView {
    fn new() -> SimpleView {
        SimpleView {
            vb: lowlevel::GLVertexBuffer::new(),
            ib: lowlevel::GLIndexBuffer::new(),
            sh: lowlevel::GLShaderProgram::new(),
            tx: lowlevel::GLTexture::new(),
        }
    }
}

impl View for SimpleView {
    type Resources = GLResources;

    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut GLResources) {
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
            let attributes = VxPos::attribute_layout_iter().map(|a| GLVertexBufferAttribute::from_layout(&a));
            self.vb.upload_data(ll, attributes, slice);
        }

        {
            let indices = [0u8, 1, 2, 0, 2, 3];

            let IndexData::Transient(slice) = indices.to_data();
            self.ib.upload_data(ll, IndexBinding::glenum_from_index_type(<u8 as IndexType>::get_layout()), slice);
        }

        {
            let raw_img = [0xabu8; 1024];

            self.tx.upload_data(ll, gl::TEXTURE_2D, 16, 16, TextureBinding::glenum_from_pixel_format(PixelFormat::Rgba8), &raw_img);
        }

        {
            let src = [
                (ShaderType::VertexShader,
                 "uniform mat4 uTrsf;
                  uniform vec3 uColor;
                  attribute vec3 vPosition;
                  varying vec3 color;
                  varying vec2 txCoord;
                  void main()
                  {
                    color = uColor;
                    txCoord = vPosition.xy;
                    gl_Position = uTrsf * vec4(vPosition, 1.0);
                  }"),
                (ShaderType::FragmentShader,
                 "varying vec3 color;
                  varying vec2 txCoord;
                  uniform sampler2D uTex;
                  void main()
                  {
                    vec3 txColor = texture2D( uTex, txCoord ).rgb;
                    vec3 col = txColor * color;
                    gl_FragColor = vec4(col, 1.0);
                  }"), ];

            self.sh.create_program(ll, src.iter());
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

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, r: &mut GLResources) {
        let ll = r.ll_mut();
        self.vb.release(ll);
        self.ib.release(ll);
        self.tx.release(ll);
        self.sh.release(ll);
    }

    fn on_surface_changed(&mut self, _ctl: &mut WindowControl, _r: &mut Self::Resources) {}

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut Self::Resources) {}

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut Self::Resources) {
        use render::lowlevel::*;

        let ll = r.ll_mut();

        gl!(ClearColor(0.0, 0.0, 0.5, 1.0));
        gl!(Clear(gl::COLOR_BUFFER_BIT));
        let size = ll.get_screen_size();
        gl!(Viewport(0,0,size.width, size.height));

        self.sh.draw(ll, gl::TRIANGLES, 0, 4,
                     |ll, loc| {});
    }

    fn on_key(&mut self, ctl: &mut WindowControl, _r: &mut Self::Resources,
              _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { ctl.close(); }
            _ => {}
        }
    }
}

#[test]
pub fn hello() {
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

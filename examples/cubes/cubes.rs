extern crate image;
#[macro_use]
extern crate shine_render as render;

use std::time::Duration;
use render::*;

mod cubeshader;

use cubeshader::*;


#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxPos {
    position: Float32x3,
    color: UInt8x4,
}


struct CubeView {
    vb: VertexBufferHandle<VxPos>,
    ib: IndexBufferHandle<u8>,
    sh: ShaderProgramHandle<CubeShader>,
}

impl CubeView {
    fn new() -> CubeView {
        CubeView {
            vb: Handle::null(),
            ib: Handle::null(),
            sh: Handle::null(),
        }
    }
}

impl View<PlatformEngine> for CubeView {
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        println!("surface ready");
        let mut queue = r.get_queue();

        let pos = [
            VxPos { position: (-1.0, 1.0, 1.0).into(), color: (0, 0, 0, 255).into() },
            VxPos { position: (1.0, 1.0, 1.0).into(), color: (0, 0, 255, 255).into() },
            VxPos { position: (-1.0, -1.0, 1.0).into(), color: (0, 255, 0, 255).into() },
            VxPos { position: (1.0, -1.0, 1.0).into(), color: (0, 255, 255, 255).into() },
            VxPos { position: (-1.0, 1.0, -1.0).into(), color: (255, 0, 0, 255).into() },
            VxPos { position: (1.0, 1.0, -1.0).into(), color: (255, 0, 255, 255).into() },
            VxPos { position: (-1.0, -1.0, -1.0).into(), color: (255, 255, 0, 255).into() },
            VxPos { position: (1.0, -1.0, -1.0).into(), color: (255, 255, 255, 255).into() },
        ];
        self.vb.create_and_set(&mut queue, &pos);

        let indices_trilist = [
            0u8, 1, 2, // 0
            1, 3, 2,
            4, 6, 5, // 2
            5, 6, 7,
            0, 2, 4, // 4
            4, 2, 6,
            1, 5, 3, // 6
            5, 7, 3,
            0, 4, 1, // 8
            4, 5, 1,
            2, 3, 6, // 10
            6, 3, 7,
        ];
        self.ib.create_and_set(&mut queue, &indices_trilist);

        /*let indices_tristrip = [
            0u8, 1, 2,
            3,
            7,
            1,
            5,
            0,
            4,
            2,
            6,
            7,
            4,
            5,
        ];
        self.ib.create_and_set(&mut queue, &indices_trilist);*/

        self.sh.create_and_compile(&mut queue);
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        println!("surface lost");
        self.vb.reset();
        self.ib.reset();
        self.sh.reset();
    }

    fn on_surface_changed(&mut self, ctl: &mut WindowControl, r: &mut PlatformBackend) {
        println!("surface changed");
        self.on_surface_lost(ctl, r);
        self.on_surface_ready(ctl, r);
    }

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {}

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        r.init_view(Some(Viewport::FullScreen),
                    Some(Float32x4(0., 0.2, 0., 1.)),
                    None);
        let mut queue = r.get_queue();

        /*float at[3]  = { 0.0f, 0.0f,   0.0f };
        float eye[3] = { 0.0f, 0.0f, -35.0f };

        float view[16];
        bx::mtxLookAt(view, eye, at);

        float proj[16];
        bx::mtxProj(proj, 60.0f, float(m_width)/float(m_height), 0.1f, 100.0f, bgfx::getCaps()->homogeneousDepth);
        bgfx::setViewTransform(0, view, proj);*/

        // Set view 0 default viewport.
        //bgfx::setViewRect(0, 0, 0, uint16_t(m_width), uint16_t(m_height) );

        let st = 0.;//self.t.sin();
        let ct = 1.;//self.t.cos();

        let params = CubeShaderParameters {
            v_position: (&self.vb, VxPos::POSITION).into(),
            v_color: (&self.vb, VxPos::COLOR).into(),
            indices: (&self.ib).into(),
            u_model_view_proj: Default::default(),
        };

        for yy in 0..11 {
            for xx in 0..11 {
                let params = params.clone();
                /*float mtx[16];
                bx::mtxRotateXY(mtx, time + xx*0.21f, time + yy*0.37f);
                mtx[12] = -15.0f + float(xx)*3.0f;
                mtx[13] = -15.0f + float(yy)*3.0f;
                mtx[14] = 0.0f;*/

                // Set model matrix for rendering.
//                bgfx::setTransform(mtx);

                self.sh.draw(&mut queue, params, Primitive::Triangle, 0, 6);
            }
        }
    }

    fn on_key(&mut self, ctl: &mut WindowControl, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { ctl.close(); }
            _ => {}
        }
    }
}


pub fn main() {
    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((1024, 1024))
        .build(&engine, CubeView::new())
        .expect("Could not initialize main window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Time(Duration::from_millis(17))) {
            break;
        }

        window.update_view();
        window.render().unwrap();
    }
}

#[macro_use]
extern crate shine_render as render;
extern crate nalgebra;
extern crate rayon;
extern crate time;

mod cubeshader;

use render::*;
use cubeshader::*;
use nalgebra::*;
use rayon::prelude::*;


#[derive(Copy, Clone, Debug, VertexDeclaration)]
struct VxPos {
    position: Float32x3,
    color: NUInt8x4,
}


struct CubeView {
    time: f32,
    vb: VertexBufferHandle<VxPos>,
    ib1: IndexBufferHandle<u8>,
    ib2: IndexBufferHandle<u8>,
    sh: ShaderProgramHandle<CubeShader>,
}

impl CubeView {
    fn new() -> CubeView {
        CubeView {
            time: 0.,
            vb: Handle::null(),
            ib1: Handle::null(),
            ib2: Handle::null(),
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

        let indices_tri_list = [
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
        self.ib1.create_and_set(&mut queue, &indices_tri_list);

        let indices_tri_strip = [
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
        self.ib2.create_and_set(&mut queue, &indices_tri_strip);

        self.sh.create_and_compile(&mut queue);
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        println!("surface lost");
        self.vb.reset();
        self.ib1.reset();
        self.ib2.reset();
        self.sh.reset();
    }

    fn on_surface_changed(&mut self, ctl: &mut WindowControl, r: &mut PlatformBackend) {
        println!("surface changed");
        self.on_surface_lost(ctl, r);
        self.on_surface_ready(ctl, r);
    }

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        self.time += 0.01;
    }

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        r.init_view(Some(Viewport::FullScreen),
                    Some(Float32x4(0., 0.2, 0., 1.)),
                    Some(1f32));
        let aspect = r.get_view_aspect();

        let eye = Point3::new(0.0, 0.0, -350.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let proj = Perspective3::new(aspect, (60f32).to_radians(), 0.1, 1000.).unwrap();

        const ROWS: usize = 100;
        const COLS: usize = 120;
        const COUNT:usize  = ROWS*COLS;

        (0..COUNT).into_par_iter()
            .for_each(|idx| {
                let mut queue = r.get_queue();

                let y = (idx / COLS) as f32;
                let x = (idx % COLS) as f32;
                let model = Isometry3::new(Vector3::new(-15.0 + x * 3.0, -15.0 + y * 3.0, 0.),
                                           Vector3::new(self.time + x * 0.21, self.time + y * 0.37, 0.));

                if idx % 2 == 0 {
                    self.sh.draw(&mut queue,
                                 CubeShaderParameters {
                                     v_position: (&self.vb, VxPos::POSITION).into(),
                                     v_color: (&self.vb, VxPos::COLOR).into(),
                                     indices: (&self.ib1).into(),
                                     u_model_view_proj: (proj * (view * model).to_homogeneous()).into(),
                                 },
                                 Primitive::Triangles, 0, 36);
                } else {
                    self.sh.draw(&mut queue,
                                 CubeShaderParameters {
                                     v_position: (&self.vb, VxPos::POSITION).into(),
                                     v_color: (&self.vb, VxPos::COLOR).into(),
                                     indices: (&self.ib2).into(),
                                     u_model_view_proj: (proj * (view * model).to_homogeneous()).into(),
                                 },
                                 Primitive::TriangleStrip, 0, 14);
                }
            });
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

    let mut frame_count = 0;
    let mut start_time = time::precise_time_s();
    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Immediate) {
            break;
        }

        window.update_view();
        window.render().unwrap();
        frame_count += 1;
        let end_time = time::precise_time_s();
        if end_time - start_time > 10f64 {
            let fps = frame_count as f64 / (end_time - start_time);
            start_time = end_time;
            frame_count = 0;
            println!("fps: {}", fps);
        }
    }
}

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
    write_mask: WriteMask,

    batch_start: f64,
    count: isize,
}

impl CubeView {
    fn new() -> CubeView {
        CubeView {
            time: 0.,
            vb: Handle::null(),
            ib1: Handle::null(),
            ib2: Handle::null(),
            sh: Handle::null(),
            write_mask: Default::default(),

            batch_start: 0.,
            count: -1,
        }
    }
}

impl CubeView {
    fn on_surface_ready(&mut self, win: &mut PlatformWindow) {
        println!("surface ready");
        if let Some(backend) = win.start_update() {
            let mut queue = backend.get_queue();

            let pos = [
                VxPos { position: (-1.0, 1.0, -1.0).into(), color: (0, 0, 0, 255).into() },
                VxPos { position: (1.0, 1.0, -1.0).into(), color: (0, 0, 255, 255).into() },
                VxPos { position: (-1.0, -1.0, -1.0).into(), color: (0, 255, 0, 255).into() },
                VxPos { position: (1.0, -1.0, -1.0).into(), color: (0, 255, 255, 255).into() },
                VxPos { position: (-1.0, 1.0, 1.0).into(), color: (255, 0, 0, 255).into() },
                VxPos { position: (1.0, 1.0, 1.0).into(), color: (255, 0, 255, 255).into() },
                VxPos { position: (-1.0, -1.0, 1.0).into(), color: (255, 255, 0, 255).into() },
                VxPos { position: (1.0, -1.0, 1.0).into(), color: (255, 255, 255, 255).into() },
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
    }

    fn on_surface_lost(&mut self, _win: &mut PlatformWindow) {
        println!("surface lost");
        self.vb.reset();
        self.ib1.reset();
        self.ib2.reset();
        self.sh.reset();
    }

    fn on_surface_changed(&mut self, win: &mut PlatformWindow) {
        println!("surface changed");
        self.on_surface_lost(win);
        self.on_surface_ready(win);
    }

    fn on_tick(&mut self, win: &mut PlatformWindow) {
        if self.count < 0 {
            let now = time::precise_time_s();
            self.batch_start = now;
            self.count = 0;
        }
        self.count += 1;
        self.time += 0.01;

        if let Some(backend) = win.start_render() {
            self.on_render(backend);
        }

        {
            let now = time::precise_time_s();
            let dt = now - self.batch_start;
            if dt > 3. {
                let fps = self.count as f64 / dt;
                println!("fps: {}", fps);
                self.batch_start = now;
                self.count = 0;
            }
        }
    }

    fn on_render(&mut self, backend: RefRender<PlatformEngine>) {
        backend.init_view(Viewport::FullScreen,
                          Some(Float32x4(0., 0.2, 0., 1.)),
                          Some(1.));
        let aspect = backend.get_pixel_aspect();

        let eye = Point3::new(0.0, 0.0, -35.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let proj = Perspective3::new(aspect, (60f32).to_radians(), 30., 60.).unwrap();

        (0..11)//.into_par_iter()
            .for_each(|yy| {
                let mut queue = backend.get_queue();

                let y = yy as f32;
                for xx in 0..11 {
                    let x = xx as f32;
                    let model = Isometry3::new(Vector3::new(-15.0 + x * 3.0, -15.0 + y * 3.0, 0.),
                                               Vector3::new(self.time + x * 0.21, self.time + y * 0.37, 0.));

                    if xx % 2 == 0 {
                        let params = CubeShaderParameters {
                            v_position: (&self.vb, VxPos::POSITION).into(),
                            v_color: (&self.vb, VxPos::COLOR).into(),
                            indices: (&self.ib1).into(),
                            u_model_view_proj: (proj * (view * model).to_homogeneous()).into(),
                            write_mask: self.write_mask,
                        };
                        self.sh.draw(&mut queue, params,
                                     Primitive::Triangles, 0, 36);
                    } else {
                        let params = CubeShaderParameters {
                            v_position: (&self.vb, VxPos::POSITION).into(),
                            v_color: (&self.vb, VxPos::COLOR).into(),
                            indices: (&self.ib2).into(),
                            u_model_view_proj: (proj * (view * model).to_homogeneous()).into(),
                            write_mask: self.write_mask,
                        };
                        self.sh.draw(&mut queue, params,
                                     Primitive::TriangleStrip, 0, 14);
                    }
                }
            });
    }

    fn on_key(&mut self, win: &mut PlatformWindow, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        println!("on_key");
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { win.close(); }
            Some(VirtualKeyCode::R) if is_down => { self.write_mask.red = !self.write_mask.red; }
            Some(VirtualKeyCode::G) if is_down => { self.write_mask.green = !self.write_mask.green; }
            Some(VirtualKeyCode::B) if is_down => { self.write_mask.blue = !self.write_mask.blue; }
            _ => {}
        }
    }
}


pub fn main() {
    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let window = render::PlatformWindowSettings::default()
        .title("main")
        .size((512, 512))
        .fb_vsync(false)
        //.fb_debug(true)
        //.fb_sub_samples(16)
        .fb_depth_bits(24, 8)
        .build(&engine,
               render::DispatchTimeout::Immediate,
               CubeView::new(),
               |window, view, cmd| {
                   match cmd {
                       &WindowCommand::SurfaceReady => view.on_surface_ready(window),
                       &WindowCommand::SurfaceLost => view.on_surface_lost(window),
                       &WindowCommand::SurfaceChanged => view.on_surface_changed(window),
                       &WindowCommand::KeyboardUp(_scan_code, virtual_key) => view.on_key(window, virtual_key, false),
                       &WindowCommand::KeyboardDown(_scan_code, virtual_key) => view.on_key(window, virtual_key, true),
                       &WindowCommand::Tick => view.on_tick(window),
                       _ => {}
                   }
               }).expect("Could not initialize main window");

    while engine.dispatch_event(render::DispatchTimeout::Infinite) {}

    drop(window);
}

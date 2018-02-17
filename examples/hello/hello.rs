extern crate shine_store as store;
extern crate shine_render as render;
extern crate rayon;

use render::*;
use rayon::prelude::*;

struct HelloView {
    time: f32,
}


impl View<PlatformEngine> for HelloView {
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {}

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {}

    fn on_surface_changed(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {}

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut PlatformBackend) {
        //use std::f32;
        self.time = (self.time + 0.01).fract();
    }

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut PlatformBackend) {
        (0..300).into_par_iter()
            .for_each(|tid| {
                r.init_view(None, Some(Float32x4((self.time * tid as f32).sin(), self.time, 0., 1.)), None);
            });
    }

    fn on_key(&mut self, ctl: &mut WindowControl, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { ctl.close(); }
            _ => {}
        }
    }
}


fn main() {
    use store::threadid;
    rayon::initialize(rayon::Configuration::new()
        .num_threads(threadid::get_preferred_thread_count())
    ).unwrap();

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((512, 512))
        .build(&engine, HelloView { time: 0.0 }).expect("Could not initialize main window");

    loop {
        if !engine.dispatch_event(render::DispatchTimeout::Immediate) {
            break;
        }

        window.update_view();
        window.render().unwrap();
    }
}

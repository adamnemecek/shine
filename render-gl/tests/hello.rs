extern crate dragorust_store as store;
extern crate dragorust_render_gl as render;
extern crate rayon;

use std::time::Duration;
use render::*;
use rayon::prelude::*;

struct SimpleView {
    time: f32,
}


impl View<PlatformEngine> for SimpleView {
    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {}

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {}

    fn on_surface_changed(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {}

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut GLBackend) {
        //use std::f32;
        self.time = (self.time + 0.01).fract();
    }

    fn on_render(&mut self, _ctl: &mut WindowControl, r: &mut GLBackend) {
        (0..300).into_par_iter()
            .for_each(|tid| {
                let mut queue = r.get_queue();
                queue.add_command(0, Command::Hello { time: (self.time * tid as f32).sin() });
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
pub fn hello_world() {
    assert!(env!("RUST_TEST_THREADS") == "1", "This test shall run in single threaded test environment: RUST_TEST_THREADS=1");

    use store::threadid;
    rayon::initialize(rayon::Configuration::new()
        .num_threads(threadid::get_preferred_thread_count())
    ).unwrap();

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let mut window = render::PlatformWindowSettings::default()
        .title("main")
        .size((512, 512))
        .build(&engine, SimpleView { time: 0.0 }).expect("Could not initialize main window");

    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((512, 512))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
        .build(&engine, SimpleView { time: 0.0 }).expect("Could not initialize sub window");

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

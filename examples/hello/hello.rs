extern crate shine_store as store;
extern crate shine_render as render;
extern crate rayon;

use render::*;
use rayon::prelude::*;
use std::time::*;

struct HelloView {
    time: f32,
}

impl HelloView {
    fn on_surface_ready(&mut self, _win: &mut PlatformWindow) {}

    fn on_surface_lost(&mut self, _win: &mut PlatformWindow) {}

    fn on_surface_changed(&mut self, _win: &mut PlatformWindow) {}

    fn on_tick(&mut self, win: &mut PlatformWindow) {
        //use std::f32;
        self.time = (self.time + 0.01).fract();

        if win.is_ready_to_render() {
            self.on_render(win);
            win.swap_buffers().unwrap();
        }
    }

    fn on_render(&mut self, win: &mut PlatformWindow) {
        let r = win.get_backend();

        (0..300).into_par_iter()
            .for_each(|tid| {
                r.init_view(None, Some(Float32x4((self.time * tid as f32).sin(), self.time, 0., 1.)), None);
            });
    }

    fn on_key(&mut self, win: &mut PlatformWindow, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { win.close(); }
            _ => {}
        }
    }
}


fn main() {
    use store::threadid;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threadid::get_preferred_thread_count())
        .build_global()
        .unwrap();

    let engine = render::PlatformEngine::new().expect("Could not initialize render engine");

    let render_timeout = render::DispatchTimeout::Time(Duration::from_millis(20));

    let window = render::PlatformWindowSettings::default()
        .title("main")
        .size((512, 512))
        .build(&engine,
               render_timeout,
               HelloView { time: 0. },
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

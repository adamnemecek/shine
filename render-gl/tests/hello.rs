//#[macro_use]
extern crate dragorust_render_gl as render;

use std::time::Duration;
use render::*;

struct SimpleView {
    t: f32
}

impl View for SimpleView {
    fn on_surface_lost(&mut self, _win: &mut Window) {}

    fn on_surface_ready(&mut self, _win: &mut Window) {}

    fn on_surface_changed(&mut self, _win: &mut Window) {}

    fn on_update(&mut self) {
        self.t += 0.01;
        if self.t >= 1.0 {
            self.t = 0.0
        }
    }

    fn on_render(&mut self, _win: &mut Window) {
        #[macro_use]
        use render::lowlevel::*;

        gl!(ClearColor(0.0, 0.0, self.t, 1.0));
        gl!(Clear(gl::COLOR_BUFFER_BIT));
    }

    fn on_key(&mut self, win: &mut Window, _scan_code: ScanCode, virtual_key: Option<VirtualKeyCode>, is_down: bool) {
        match virtual_key {
            Some(VirtualKeyCode::Escape) if !is_down => { win.close(); }
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
        .build(&engine, SimpleView { t: 0.0 }).expect("Could not initialize main window");

    let mut sub_window = render::PlatformWindowSettings::default()
        .title("sub")
        .size((256, 256))
        //.extra(|e| { e.gl_profile(render::opengl::OpenGLProfile::ES2); })
        .build(&engine, SimpleView { t: 0.5 }).expect("Could not initialize sub window");

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

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
    //PlatformRenderManager,
    vb: lowlevel::GLVertexBuffer,
    ib: lowlevel::GLIndexBuffer,
}

impl SimpleView {
    fn new() -> SimpleView {
        SimpleView {
            //r: PlatformRenderManager::new(),
            vb: lowlevel::GLVertexBuffer::new(),
            ib: lowlevel::GLIndexBuffer::new(),
        }
    }
}

impl View for SimpleView {
    type Resources = GLResources;

    fn on_surface_ready(&mut self, _ctl: &mut WindowControl, r: &mut GLResources) {
        use lowlevel::*;

        let pos = [
            VxPos { position: f32x3!(1, 0, 0) },
            VxPos { position: f32x3!(1, 1, 0) },
            VxPos { position: f32x3!(0, 1, 0) },
            VxPos { position: f32x3!(0, 0, 0) },
        ];

        {
            let VertexData::Transient(slice) = pos.to_data();
            let mut attributes = GLVertexBufferAttributeVec::new();
            for idx in VxPos::get_attributes() {
                attributes.push(GLVertexBufferAttribute::from_layout(&VxPos::get_attribute_layout(*idx)));
            }
            self.vb.upload_data(r.ll_mut(), attributes, slice);
        }

        {
            let indices = [0u8, 1, 2, 0, 2, 3];
            let IndexData::Transient(slice) = indices.to_data();
            self.ib.upload_data(r.ll_mut(), IndexBinding::glenum_from_index_type(<u8 as IndexType>::get_layout()), slice);
        }
    }

    fn on_surface_lost(&mut self, _ctl: &mut WindowControl, r: &mut GLResources) {
        self.vb.release(r.ll_mut());
        self.ib.release(r.ll_mut());
    }

    fn on_surface_changed(&mut self, _ctl: &mut WindowControl, _r: &mut Self::Resources) {}

    fn on_update(&mut self, _ctl: &mut WindowControl, _r: &mut Self::Resources) {}

    fn on_render(&mut self, _ctl: &mut WindowControl, _r: &mut Self::Resources) {}

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

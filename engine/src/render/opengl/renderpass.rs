use std::rc::Rc;
use std::cell::{RefCell};

use render::*;
use render::opengl::lowlevel::*;

/// Structure to store hardware data associated to a RenderPass.
struct GLRenderPassData {}

impl GLRenderPassData {
    pub fn new() -> GLRenderPassData {
        GLRenderPassData {}
    }
}

impl Drop for GLRenderPassData {
    fn drop(&mut self) {}
}


/// RenderCommand to clear the frame buffers
struct ClearCommand {
    view_port: pass::ViewPort,
    clear_color: Option<Float32x3>,
}

impl Command for ClearCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        gl_check_error();

        //todo: handle render target size vs viewport
        let fb_size = ll.get_screen_size();

        match self.view_port {
            pass::ViewPort::None => {}
            pass::ViewPort::PixelRectangle(rect) => unsafe { gl::Viewport(rect.position.x, rect.position.y, rect.size.width, rect.size.height); }
            pass::ViewPort::Fullscreen => unsafe { gl::Viewport(0, 0, fb_size.width, fb_size.height); }
        }

        if let Some(clear_color) = self.clear_color {
            unsafe {
                gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }

        gl_check_error();
    }
}

/// RenderPass implementation for OpenGL.
pub struct GLRenderPass(Rc<RefCell<GLRenderPassData>>);

impl GLRenderPass {
    pub fn new() -> GLRenderPass {
        GLRenderPass(
            Rc::new(RefCell::new(GLRenderPassData::new()))
        )
    }

    pub ( crate ) fn prepare(&mut self, queue: &mut CommandStore, meta_index: ActivePassIndex, config: &RenderPassConfig) {
        // todo: if this branching optional matching takes too much time, it can be split up
        // into multiple commands

        let clear_color = match config.clear {
            pass::Clear::Frame(color) => Some(color),
            pass::Clear::None => None,
        };

        let cmd = ClearCommand {
            clear_color: clear_color,
            view_port: config.view_port,
        };

        queue.add((meta_index, cmd.get_sort_key()), cmd);
    }
}


pub type RenderPassImpl = GLRenderPass;

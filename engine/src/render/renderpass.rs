#![deny(missing_docs)]

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;

use render::*;

/// Geometry primitive type
#[derive(Copy, Clone, Debug)]
pub enum Primitive {
    /// Point primitive (1 vertex per primitive)
    Point,
    /// Line primitive (2 vertex per primitive)
    Line,
    /// Triangle primitive (3 vertex per primitive)
    Triangle,
}


/// Settings structure for RenderPass behavior.
///
/// This structure stores everything that can be customized when
/// constructing a render pass.
#[derive(Clone)]
pub struct RenderPassConfig {
    screen: bool,
}

impl RenderPassConfig {
    /// Creates render pass  settings with defaults.
    pub fn new() -> RenderPassConfig {
        RenderPassConfig {
            screen: true,
        }
    }
}


/// Structure to store the render pass abstraction.
pub struct RenderPass {
    platform: RenderPassImpl,
    command_store: Rc<RefCell<CommandStore>>,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(_: RenderPassConfig, command_store: Rc<RefCell<CommandStore>>) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            command_store: command_store,
        }
    }

    /// Submits a geometry for rendering
    pub fn draw(&mut self, vertices: &VertexBuffer, primitive: Primitive, start: usize, vertex_count: usize) {
        self.platform.draw(self.command_store.borrow_mut().deref_mut(), &vertices.platform, primitive, start, vertex_count);
    }
}

impl CommandQueue for RenderPass {
    fn add<C: Command + 'static>(&mut self, cmd: C) {
        self.command_store.borrow_mut().add(cmd);
    }
}


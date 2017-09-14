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
#[derive(Copy, Clone)]
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
    config: RenderPassConfig,
    command_store: Rc<RefCell<CommandStore>>,
    order_index: usize,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(order_index: usize, command_store: Rc<RefCell<CommandStore>>) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            config: RenderPassConfig::new(),
            command_store: command_store,
            order_index: order_index,
        }
    }

    pub ( crate ) fn get_order_index(&self) -> usize {
        self.order_index
    }

    /// Returns the current config for mutation.
    ///
    /// Configuration can be altered any time, only the state at the time of submitting
    /// counts.
    pub fn config_mut(&mut self) -> &mut RenderPassConfig {
        &mut self.config
    }

    /// Sets up the pass for rendering.
    ///
    /// This function is responsible to set up the pass for rendering. It generates commands to
    /// clear buffers, set viewports, bind render targets, etc.
    pub fn prepare(&mut self) {
        self.platform.prepare(self.command_store.borrow_mut().deref_mut(), config);
    }

    /// Sends a geometry for rendering
    pub fn draw(&mut self, vertices: &VertexBuffer, primitive: Primitive, start: usize, vertex_count: usize) {
        self.platform.draw(self.command_store.borrow_mut().deref_mut(), &vertices.platform, primitive, start, vertex_count);
    }
}

impl CommandQueue for RenderPass {
    fn add<C: Command + 'static>(&mut self, cmd: C) {
        self.command_store.borrow_mut().add(cmd);
    }
}

#![deny(missing_docs)]

use std::rc::Rc;
use std::cell::RefCell;

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


/// Structure to store the render pass abstraction.
pub struct RenderPass {
    platform: RenderPassImpl,
    config: RenderPassConfig,
    command_store: Rc<RefCell<CommandStore>>,
    pub ( crate ) activation_index: ActivePassIndex,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(command_store: Rc<RefCell<CommandStore>>) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            config: RenderPassConfig::new(),
            command_store: command_store,
            activation_index: ActivePassIndex::new(),
        }
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
        self.platform.prepare(&mut self.command_store.borrow_mut(), self.activation_index, &self.config);
    }
}

impl CommandQueue for RenderPass {
    fn add<C: Command + 'static>(&mut self, cmd: C) {
        let sort_key = cmd.get_sort_key() + 1;
        self.command_store.borrow_mut().add((self.activation_index, sort_key), cmd);
    }
}

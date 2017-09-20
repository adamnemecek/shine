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


/// Structure to store the render pass abstraction.
pub struct RenderPass {
    platform: RenderPassImpl,
    config: RenderPassConfig,
    command_store: Rc<RefCell<CommandStore>>,
    pub ( crate ) meta_index: PassMetaIndex,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(command_store: Rc<RefCell<CommandStore>>) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            config: RenderPassConfig::new(),
            command_store: command_store,
            meta_index: PassMetaIndex::new(),
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
        self.platform.prepare(self.command_store.borrow_mut().deref_mut(), &self.config);
    }
}

impl CommandQueue for RenderPass {
    fn add<C: Command + 'static>(&mut self, cmd: C) {
        self.command_store.borrow_mut().add(cmd);
    }
}

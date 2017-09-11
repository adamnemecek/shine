#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use render::*;

/// Structure to abstract the command queue
pub trait CommandQueue {
    /// Returns the command store
    fn get_command_store(&self) -> RefMut<CommandStoreImpl>;
}


/// Structure to manage multi-pass rendering.
///
/// Usually this struct is used to handle the rendering.
pub struct RenderManager<P: Eq + Hash> {
    platform: RenderManagerImpl,
    passes: HashMap<P, RefCell<RenderPass>>,
    command_store: Rc<RefCell<CommandStoreImpl>>,
    //order: Vec<RefCell<RenderPass>>,
}

impl<P: Eq + Hash> RenderManager<P> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<P> {
        RenderManager {
            platform: RenderManagerImpl::new(),
            passes: HashMap::new(),
            command_store: CommandStoreImpl::new(),
        }
    }

    /// Creates a new pass (leaving only for a single frame)
    ///pub fn create_pass(id: P) -> PassBuilder{}
    ///

    /// Gets an existing render pass.
    ///
    /// If pass was not created in the current frame, None is returned
    pub fn get(&mut self, id: P) -> Option<RefMut<RenderPass>> {
        /*self.passes.entry(id)
            .or_insert(RefCell::new(RenderPass::new(0)))
            .borrow_mut()*/
        None
    }


    /// Order passes by the dependency graph
    fn sort_passes(&mut self) {
        /*let mut i = 0u8;
        for ref mut pass in self.passes.values_mut() {
            //pass.borrow_mut().set_order(i);
            i = i + 1;
        }*/
    }

    /// Submit a command queue for rendering.
    pub fn submit(&mut self, window: &Window) {
        self.platform.submit(window.platform_mut().deref_mut(), self.command_store.borrow_mut());
    }
}


impl<P: Eq + Hash> CommandQueue for RenderManager<P> {
    /// Returns the command queue
    fn get_command_store(&self) -> RefMut<CommandStoreImpl> {
        self.command_store.borrow_mut()
    }
}

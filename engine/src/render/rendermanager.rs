#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use render::*;

/// Trait to index RenderPasses.
///
/// It is usally some enum but other trait can be used.
pub trait PassKey: Eq + Copy + Clone + Hash + 'static {}


/// Helper to construct a pass using the factory pattern.
pub struct RenderPassBuilder<'a, K: PassKey> {
    manager: &'a mut RenderManager<K>,
    id: K,
    config: RenderPassConfig,
}

impl<'a, K: PassKey> RenderPassBuilder<'a, K> {
    /// Builds the configured render pass.
    ///
    /// # Error
    ///
    /// If render pass cannot be created None is returned:
    ///   - The pass already exists, use find_pass instead
    ///   - Incompatible configuration
    pub fn build(&mut self) -> Option<RefMut<RenderPass>> {
        use std::collections::hash_map::Entry::*;

        match self.manager.passes.entry(self.id) {
            Occupied(_) => None,
            e => Some(e.or_insert(RefCell::new(RenderPass::new(self.config.clone(), self.manager.command_store.clone()))).borrow_mut()),
        }
    }
}


/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassKey> {
    platform: RenderManagerImpl,
    passes: HashMap<K, RefCell<RenderPass>>,
    command_store: Rc<RefCell<CommandStore>>,
    //order: Vec<RefCell<RenderPass>>,
}

impl<K: PassKey> RenderManager<K> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K> {
        RenderManager {
            platform: RenderManagerImpl::new(),
            passes: HashMap::new(),
            command_store: Rc::new(RefCell::new(CommandStore::new())),
        }
    }

    /// Creates a new pass with the given id.
    ///
    /// Passes are a short leaving objects and the pass-graph have to be recreated for each frame.
    pub fn create_pass<'a>(&'a mut self, id: K) -> RenderPassBuilder<'a, K> {
        RenderPassBuilder {
            manager: self,
            id: id,
            config: RenderPassConfig::new()
        }
    }


    /// Gets an existing render pass.
    ///
    /// If pass was not created in the current frame, None is returned
    pub fn find_pass(&mut self, id: K) -> Option<RefMut<RenderPass>> {
        self.passes.get(&id).map(|ref e| e.borrow_mut())
    }

    /// Sends commands for processing.
    pub fn submit(&mut self, window: &Window) {
        self.sort_passes();

        let ref mut commands = *self.command_store.borrow_mut();
        //commands.sort(self.pass_order);
        window.platform().process_commands(commands.iter_mut());

        commands.clear();
    }


    /// Order passes by the dependency graph
    fn sort_passes(&mut self) {
        /*let mut i = 0u8;
        for ref mut pass in self.passes.values_mut() {
            //pass.borrow_mut().set_order(i);
            i = i + 1;
        }*/
    }
}


impl<K: PassKey> CommandQueue for RenderManager<K> {
    fn add<C: Command>(&mut self, cmd: C) {
        self.command_store.borrow_mut().add(cmd);
    }
}

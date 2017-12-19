#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::sync::Mutex;
use common::*;

pub struct AA<K: PassId, D> {
    pub k: K,
    pub d: D,
}

/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassId, R: ResourceManager> {
    resources: R,
    command_store: R::CommandStore,
    pass_manager: Mutex<PassManager<K, R>>,
}

impl<K: PassId, R: ResourceManager> RenderManager<K, R> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K, R> {
        RenderManager {
            resources: R::new(),
            command_store: R::CommandStore::new(),
            pass_manager: Mutex::new(PassManager::new()),
        }
    }

    /// Get pass by the id
    pub fn get_pass(&self, id: K) -> &Pass<R> {
        let mut pass_manager = self.pass_manager.lock().unwrap();
        let pass = pass_manager.get_pass(id, &self.resources, &self.command_store);

        // Passes are stored on heap and once a pass is created it won't be changed any more,
        // thus it is safe
        unsafe {
            &(*(pass as *const Pass<R>))
        }
    }
}

impl<K: PassId, R: ResourceManager> CommandQueue for RenderManager<K, R> {
    fn add<C: Command>(&self, cmd: C) {
        let cmd_key = cmd.get_sort_key();
        self.command_store.add((ActivePassIndex::inactive(), cmd_key), cmd);
    }
}
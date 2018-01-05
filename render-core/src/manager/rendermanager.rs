#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::sync::Mutex;
use resources::*;
use manager::*;


/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassId, R: Resources> {
    command_store: CommandStore<R::Command>,
    pass_manager: Mutex<PassManager<K, R>>,
}

impl<K: PassId, R: Resources> RenderManager<K, R> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K, R> {
        RenderManager {
            command_store: CommandStore::new(),
            pass_manager: Mutex::new(PassManager::new()),
        }
    }

    /// Get pass by the id
    pub fn get_pass(&self, id: K) -> &Pass<R> {
        let mut pass_manager = self.pass_manager.lock().unwrap();
        let pass = pass_manager.get_pass(id, &self.command_store);

        // Passes are stored on heap and once a pass is created it won't be changed any more,
        // thus it is safe
        unsafe {
            &(*(pass as *const Pass<R>))
        }
    }
}

impl<K: PassId, R: Resources> CommandQueue for RenderManager<K, R> {
    type Command = R::Command;

    fn add(&self, cmd: R::Command) {
        let command_store = &self.command_store as *const CommandStore<R::Command> as *mut CommandStore<R::Command>;
        unsafe { &mut *command_store }.add(ActivePassIndex::inactive(), cmd);
    }
}
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use container::ops::At;
use render::*;


/// Enum to store the error occurred during rendering
#[derive(Debug, Clone)]
pub enum Error {
    /// Error reported during a pass creation.
    PassCreationError(String),
    /// Error reported by the OS during rendering
    ContextError(String),
    /// Context is lost, ex window has been closed.
    ContextLost,
}


/// Trait to index RenderPasses.
///
/// It is usally some enum but other trait can be used.
pub trait PassKey: 'static + Debug + Copy + Clone + Eq + Hash {}


/// Trait to query the order of a pass in the pipeline
#[derive(Copy, Clone, Debug)]
pub ( crate ) struct QuerySortedOrder(usize);


/// Sturcture to store meta-info of render passes.
struct PassMeta {
    order: usize,
    last_use: usize,
}

impl Default for PassMeta {
    fn default() -> PassMeta {
        PassMeta {
            order: usize::max_value(),
            last_use: 0,
        }
    }
}


/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassKey> {
    platform: RenderManagerImpl,
    passes: HashMap<K, RefCell<RenderPass>>,
    passes_meta: RefCell<Vec<PassMeta>>,
    command_store: Rc<RefCell<CommandStore>>,
    submit_counter: usize,
}

impl<K: PassKey> RenderManager<K> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K> {
        RenderManager {
            platform: RenderManagerImpl::new(),
            passes: HashMap::new(),
            passes_meta: RefCell::new(vec!()),
            command_store: Rc::new(RefCell::new(CommandStore::new())),
            submit_counter: 0,
        }
    }

    /// Creates a new pass with the given id.
    ///
    /// Passes are a short leaving objects and the pass-graph have to be recreated for each frame.
    pub fn create_pass<'a>(&'a mut self, id: K) -> Result<RefMut<RenderPass>, Error> {
        use std::collections::hash_map::Entry::*;

        let idx = self.passes.len();
        match self.passes.entry(id) {
            Occupied(_) => Err(Error::PassCreationError(format!("Pass {:?} already exists", id))),
            e => Ok(e.or_insert(RefCell::new(RenderPass::new(idx, self.command_store.clone())))),
        }
    }

    /// Gets an existing render pass.
    ///
    /// If pass was not created in the current frame, None is returned
    pub fn find_pass(&self, id: K) -> Option<RefMut<RenderPass>> {
        self.passes.get(&id).map(|ref pass| {
            let pass = pass.borrow_mut();
            let ref mut meta = self.passes_meta.borrow_mut()[pass.get_order_index()];
            if meta.last_use != self.submit_counter {
                meta.last_use = self.submit_counter;
                pass.prepare();
            }
            pass
        })
    }

    /// Sends commands for processing.
    pub fn submit(&mut self, window: &Window) {
        self.sort_passes();

        {
            let ref mut commands = *self.command_store.borrow_mut();
            commands.sort(self);
            window.platform().process_commands(commands.iter_mut());
            commands.clear();
        }

        self.submit_counter += 1;
    }


    /// Order passes by the dependency graph
    fn sort_passes(&mut self) {
        let ref mut passes_meta = self.passes_meta.borrow_mut();
        passes_meta.resize_default(self.passes.len());

        let mut i = 0;
        for pass in self.passes.iter() {
            let ref mut meta = passes_meta[pass.1.borrow().get_order_index()];
            meta.order = i;
            i += 1;
        }
    }
}

impl<K: PassKey> CommandQueue for RenderManager<K> {
    fn add<C: Command>(&mut self, cmd: C) {
        self.command_store.borrow_mut().add(cmd);
    }
}

impl<K: PassKey> At<QuerySortedOrder> for RenderManager<K> {
    type Output = usize;

    fn at(&self, idx: QuerySortedOrder) -> usize {
        self.passes_meta.borrow()[idx.0].order
    }
}

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use container::ops::At;
use render::*;

/// Trait to index RenderPasses.
///
/// It is usally some enum but other trait can be used.
pub trait PassKey: 'static + Debug + Copy + Clone + Eq + Hash {}


/// Trait to index the active passes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActivePassIndex {
    id: usize,
    time_stamp: usize,
}

impl ActivePassIndex {
    /// Creates an invalid index.
    pub fn new() -> ActivePassIndex {
        ActivePassIndex {
            id: usize::max_value(),
            time_stamp: usize::max_value()
        }
    }

    /// Checks if index is valid valid for the current frame.
    pub fn check_time_stamp(&self, current_time: usize) -> bool {
        self.time_stamp == current_time
    }

    /// Returns the index as an usize.
    pub fn as_index(&self) -> usize {
        self.id
    }
}

/// Trait to index the passes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub ( crate ) struct PassIndex(usize);

impl PassIndex {
    pub fn as_index(&self) -> usize {
        self.0
    }
}


/// Structure to store activation info for the render passes.
struct ActivePass {
    _index: PassIndex,
    order: usize,
}

/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassKey> {
    passes: Vec<RefCell<RenderPass>>,
    passes_lookup: HashMap<K, (PassIndex, ActivePassIndex)>,
    active_passes: Vec<ActivePass>,

    command_store: Rc<RefCell<CommandStore>>,
    current_time: usize,
}

impl<K: PassKey> RenderManager<K> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K> {
        RenderManager {
            passes: vec!(),
            passes_lookup: HashMap::new(),
            active_passes: vec!(),
            command_store: Rc::new(RefCell::new(CommandStore::new())),
            current_time: 0,
        }
    }

    /// Gets or creates a pass with the given id.
    ///
    /// By default passes are activated only for a single frame and whenever  when a pass is queried, it is reactivated
    /// automatically for the current frame.
    pub fn get_pass(&mut self, id: K) -> RefMut<RenderPass> {
        // get or create the pass corrsponding to the given id
        let entry = {
            let passes = &mut self.passes;
            let command_store = self.command_store.clone();

            let entry = self.passes_lookup.entry(id);
            entry.or_insert_with(|| {
                passes.push(RefCell::new(RenderPass::new(command_store)));
                (PassIndex(passes.len() - 1), ActivePassIndex::new())
            })
        };

        // find the index of the pass
        let mut pass = self.passes[entry.0.as_index()].borrow_mut();

        if !entry.1.check_time_stamp(self.current_time) {
            // first access to this pass in this frame
            let activation_index = ActivePassIndex { id: self.active_passes.len(), time_stamp: self.current_time };
            pass.activation_index = activation_index;
            entry.1 = activation_index;
            self.active_passes.push(ActivePass {
                _index: entry.0,
                order: 0,
            });
        }

        pass
    }

    /// Sends commands for processing.
    pub fn submit(&mut self, window: &mut Window) {
        self.sort_passes();

        {
            let ref mut commands = *self.command_store.borrow_mut();
            commands.sort(self);
            commands.process(window.platform_mut());
            commands.clear();
        }

        // clear active passes
        self.active_passes.clear();
        self.current_time += 1;
    }

    /// Order passes by the dependency graph
    fn sort_passes(&mut self) {
        // call prepare for the active passes
        for pass in self.passes.iter() {
            let mut pass = pass.borrow_mut();
            if pass.activation_index.check_time_stamp(self.current_time) {
                pass.prepare();
            }
        }

        // todo: topology sort passes
        let mut i = 0;
        for ref mut active_pass in self.active_passes.iter_mut() {
            active_pass.order = i;
            i += 1;
        }
    }
}

impl<K: PassKey> CommandQueue for RenderManager<K> {
    fn add<C: Command>(&mut self, cmd: C) {
        let cmd_key = cmd.get_sort_key();
        self.command_store.borrow_mut().add((ActivePassIndex::new(), cmd_key), cmd);
    }
}

impl<K: PassKey> At<ActivePassIndex> for RenderManager<K> {
    type Output = usize;

    fn at(&self, idx: ActivePassIndex) -> usize {
        if idx == ActivePassIndex::new() {
            0
        } else {
            assert!(idx.check_time_stamp(self.current_time), "Pass was not activated in the current frame");
            self.active_passes[idx.as_index()].order + 1
        }
    }
}

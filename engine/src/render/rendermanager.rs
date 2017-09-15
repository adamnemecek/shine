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
pub ( crate ) struct PassMetaIndex(usize);

impl PassMetaIndex {
    pub fn new() -> PassMetaIndex {
        PassMetaIndex(usize::max_value())
    }

    pub fn as_index(&self) -> usize {
        self.0
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


/// Sturcture to store meta-info of render passes.
struct PassMeta {
    index: PassIndex,
    order: usize,
}

/// Structure to manage multi-pass rendering.
pub struct RenderManager<K: PassKey> {
    passes: Vec<RefCell<RenderPass>>,
    passes_lookup: HashMap<K, (PassIndex, PassMetaIndex)>,
    active_passes: Vec<PassMeta>,

    command_store: Rc<RefCell<CommandStore>>,
    submit_counter: usize,
}

impl<K: PassKey> RenderManager<K> {
    /// Creates a new renderer.
    pub fn new() -> RenderManager<K> {
        RenderManager {
            passes: vec!(),
            passes_lookup: HashMap::new(),
            active_passes: vec!(),
            command_store: Rc::new(RefCell::new(CommandStore::new())),
            submit_counter: 0,
        }
    }

    /// Gets or creates a pass with the given id.
    ///
    /// By default passes are activated only for a single frame and whenever  when a pass is queried, it is reactivated
    /// automatically for the current frame.
    pub fn get_pass(&mut self, id: K) -> RefMut<RenderPass> {
        // get or create entry for id
        let entry = {
            let passes = &mut self.passes;
            let command_store = self.command_store.clone();

            let entry = self.passes_lookup.entry(id);
            entry.or_insert_with(|| {
                passes.push(RefCell::new(RenderPass::new(command_store)));
                (PassIndex(passes.len() - 1), PassMetaIndex::new())
            })
        };

        // find the index of the pass
        let mut pass = self.passes[entry.0.as_index()].borrow_mut();

        if entry.1 == PassMetaIndex::new() {
            // pass have to be activated, firs access in this frame
            let meta_index = PassMetaIndex(self.active_passes.len());
            pass.meta_index = meta_index;
            entry.1 = meta_index;
            self.active_passes.push(PassMeta {
                index: entry.0,
                order: 0,
            });
        }

        pass
    }

    /// Sends commands for processing.
    pub fn submit(&mut self, window: &Window) {
        self.sort_passes();

        {
            let ref mut commands = *self.command_store.borrow_mut();
            commands.sort(self);
            window.platform().process(|ll| {
                for ref mut cmd in commands.iter_mut() {
                    cmd.process(ll);
                }
            });
            commands.clear();
        }

        // deactive passes
        self.active_passes.clear();
        for lookup in self.passes_lookup.iter_mut() {
            (lookup.1).1 = PassMetaIndex::new();
        }

        self.submit_counter += 1;
    }


    /// Order passes by the dependency graph
    fn sort_passes(&mut self) {
        // call prepare for the active passes
        for meta in self.active_passes.iter() {
            let mut pass = self.passes[meta.index.as_index()].borrow_mut();
            pass.prepare();
        }

        // todo: topology sort passes
        let mut i = 0;
        for ref mut meta in self.active_passes.iter_mut() {
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

impl<K: PassKey> At<PassMetaIndex> for RenderManager<K> {
    type Output = usize;

    fn at(&self, idx: PassMetaIndex) -> usize {
        self.active_passes[idx.as_index()].order
    }
}

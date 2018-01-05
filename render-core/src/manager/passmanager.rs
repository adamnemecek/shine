use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::marker::PhantomData;

use resources::*;
use manager::*;

pub trait PassId: 'static + Clone + Hash + Eq + Debug {}

/// Structure to store the render pass abstraction.
pub struct Pass<R: Resources> {
    command_store: *mut CommandStore<R::Command>,
    index: ActivePassIndex,
    _ph: PhantomData<R>
}

impl<R: Resources> Pass<R> {
    fn new(command_store: &CommandStore<R::Command>) -> Pass<R> {
        Pass {
            command_store: command_store as *const CommandStore<R::Command> as *mut CommandStore<R::Command>,
            index: ActivePassIndex::inactive(),
            _ph: PhantomData,
        }
    }

    pub fn activate(&mut self, index: ActivePassIndex) {
        self.index = index;
        //self.render_target.prepare(self);
    }

    pub fn configure(&self, _cfg: RenderTargetConfig) {
        //self.render_target.configure(self, cfg);
    }
}

impl<R: Resources> CommandQueue for Pass<R> {
    type Command = R::Command;

    fn add(&self, cmd: R::Command) {
        unsafe { &mut *self.command_store }.add(self.index, cmd);
    }
}


/// Index into the available passes vector
#[derive(Copy, Clone, Debug)]
struct PassIndex(usize);


/// Index into the active passes vector
#[derive(Copy, Clone, Debug)]
pub struct ActivePassIndex(usize);

impl ActivePassIndex {
    pub fn inactive() -> ActivePassIndex {
        ActivePassIndex(usize::max_value())
    }

    pub fn active(idx: usize) -> ActivePassIndex {
        ActivePassIndex(idx)
    }

    pub fn is_active(&self) -> bool {
        self.0 != usize::max_value()
    }
}


/// Meta-data for the active passes
struct ActivePass {
    /// Index into the available passes vector
    index: usize,

    /// The order of the passes according to topology sort
    order: usize,
}


/// Manage passes
pub struct PassManager<K: PassId, R: Resources> {
    passes: Vec<Box<Pass<R>>>,
    passes_lookup: HashMap<K, (PassIndex, ActivePassIndex)>,
    active_passes: Vec<ActivePass>,
}

impl<K: PassId, R: Resources> PassManager<K, R> {
    pub fn new() -> PassManager<K, R> {
        PassManager {
            passes: vec!(),
            passes_lookup: HashMap::new(),
            active_passes: vec!(),
        }
    }

    /// Gets or creates a pass with the given id.
    ///
    /// By default passes are activated only for a single frame and whenever a pass is acquired from the
    /// manager, it is activated automatically.
    pub fn get_pass(&mut self, id: K, command_store: &CommandStore<R::Command>) -> &Pass<R> {
        // Get the pass by the provided key.
        // If pass is not defined yet, a new one is created
        let entry = {
            let passes = &mut self.passes;
            let passes_lookup = &mut self.passes_lookup;
            let active_passes = &self.active_passes;

            let entry = passes_lookup.entry(id);
            entry.or_insert_with(|| {
                passes.push(Box::new(Pass::new(command_store)));
                (PassIndex(passes.len() - 1, ), ActivePassIndex::active(active_passes.len()))
            })
        };

        let (pass_idx, active_idx) = ((entry.0).0, (entry.1).0);

        // find or create the active pass
        if active_idx >= self.active_passes.len() {
            self.active_passes.push(ActivePass {
                index: pass_idx,
                order: 0
            });
        }
        assert!(active_idx < self.active_passes.len());

        self.passes[pass_idx].as_ref()
    }

    /// Sort passes by topology order.
    pub fn sort(&mut self) {
        // Activate each active passes.
        // Order is not important as activation generates commands those are sorted
        for (idx, ref active_pass) in self.active_passes.iter_mut().enumerate() {
            self.passes[active_pass.index].activate(ActivePassIndex::active(idx));
        }

        // todo: topology sort passes
        let mut i = 0;
        for ref mut active_pass in self.active_passes.iter_mut() {
            active_pass.order = i;
            i += 1;
        }
    }

    /// Sort passes by topology order.
    pub fn clear_active(&mut self) {
        self.active_passes.clear();
    }
}

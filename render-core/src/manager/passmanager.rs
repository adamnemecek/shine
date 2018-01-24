/*
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
//use std::marker::PhantomData;

use resources::*;
use manager::*;
//use store::fjsqueue::*;


pub trait PassId: 'static + Clone + Hash + Eq + Debug {}

struct PassData<E: Engine> {}


/// Structure to store the render pass abstraction.
pub struct Pass<'p, E: Eninge> {
    data: &mut PassData<E>,
    index: ActivePassIndex,
    _ph: PhantomData<E>,
}

impl<'p, E: Engine> Pass<'p, E> {
    fn new<'p>() -> Pass<'p, E> {
        Pass {
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


pub struct PassManager<K: PassId, E: Engine> {
    passes: HashMap<K, (usize, usize)>,

    active_passes: Vec<K>,
}

impl<K: PassId, E: Engine> PassManager<K, E> {
    pub fn new() -> PassManager<K, E> {
        PassManager {
            passes: HashMap::new(),
            active_passes: vec!(),
        }
    }

    /// Gets or creates a pass with the given id.
    ///
    /// By default passes are activated only for a single frame and whenever a pass is acquired from the
    /// manager, it is activated automatically.
    pub fn get_pass<'p>(&'p mut self, id: K, command_store: &'p E: FrameCompose) -> Pass<'p, E> {
        let passes = &mut self.passes;
        let passes_lookup = &mut self.passes_lookup;
        let active_passes = &self.active_passes;

        let entry = passes.entry(id).or_insert(PassData {});
        if entry.1.index == 0 {
            entry.1.index = active_passes.size();
            active_passes.push(id);
        }



        let (pass_idx, active_idx) = ((entry.0).0, (entry.1).0);

        // find or create the active pass
        if active_idx > = self.active_passes.len() {
            self.active_passes.push(ActivePass {
                index: pass_idx,
                order: 0,
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
* /
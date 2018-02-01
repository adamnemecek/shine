use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
//use std::marker::PhantomData;

use resources::*;
use manager::*;
//use store::fjsqueue::*;


pub trait PassId: 'static + Clone + Hash + Eq + Debug {}

struct PassData<E: Engine> {
    //active_index: usize
}


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

struct PassId {
    pass: usize,
    active: usize,
}

struct ActivePassId {
    pass: usize,
    order: usize,
}


pub struct PassManager<K: PassId, E: Engine> {
    passes: Vec<PassData>,
    active_passes: Vec<ActivePass>,
    passes_lookup: HashMap<K, PassId>,
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
    pub fn get_pass<'p>(&'p mut self, id: K, command_store: &'p E: CommandQueue) -> Pass<'p, E> {
        let entry = {
            let passes = &mut self.passes;
            let passes_lookup = &mut self.passes_lookup;
            let active_passes = &self.active_passes;

            passes_lookup.entry(id).or_insert_with(|| {
                let id = passes.size();
                passes.push(PassData {});
                PassId {
                    pass: id,
                    active: max_value(),
                }
            });
        };

        // find or create the active pass
        if entry.active >= self.active_passes.len() {
            entry.active = self.active_passes.size();
            self.active_passes.push(ActivePassId {
                pass: entry.pass,
                order: 0,
            });
        }

        Pass {

        }
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
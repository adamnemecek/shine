#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use render::*;

/// Structure to manage multi-pass rendering
pub struct PassManager<P: Eq + Hash> {
    passes: HashMap<P, RefCell<RenderPass>>,
    //order: Vec<RefCell<RenderPass>>,
}

impl<P: Eq + Hash> PassManager<P> {
    /// Creates a new renderer.
    pub fn new() -> PassManager<P> {
        PassManager {
            passes: HashMap::new()
        }
    }

    /// Acquire a render pass.
    ///
    /// If pass is not present yet, a new pass is created otherwise, the
    /// reference of the old pass is returned.
    pub fn get(&mut self, id: P) -> RefMut<RenderPass> {
        self.passes.entry(id)
            .or_insert(RefCell::new(RenderPass::new(0)))
            .borrow_mut()
    }


    /// Order passes by the dependency graph
    pub fn sort_passes(&mut self) {
        /*let mut i = 0u8;
        for ref mut pass in self.passes.values_mut() {
            //pass.borrow_mut().set_order(i);
            i = i + 1;
        }*/
    }
}



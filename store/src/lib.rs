#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]
#![feature(drain_filter)]
#![feature(attr_literals)]

#[macro_use]
extern crate lazy_static;

extern crate num_cpus;


/// Define engine limitations
pub mod libconfig {
    /// Maximum number of threads that can be used at once.
    /// Set to 0 find some "optimal" value based on the available number of logical cores.
    /// The allocated thread id's cannot exceed this hard limit, #see threadid.
    pub const MAX_THREAD_COUNT: usize = 0;

    /// Preferred number of threads used for data/task processing
    /// Set to 0 find some "optimal" value based on the available number of logical cores.
    pub const PREFERRED_THREAD_COUNT: usize = 0;
}

pub mod arena;
pub mod store;
pub mod hashstore;

pub mod threadid;
pub mod spscstate;

pub mod forkjoinsortedqueue;

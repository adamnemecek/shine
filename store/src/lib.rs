#![feature(align_offset)]
#![feature(drain_filter)]

extern crate lazy_static;
extern crate log;
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

mod indexedarena;
mod stablearena;

pub mod arena {
    pub use super::indexedarena::*;
    pub use super::stablearena::*;
}

pub mod hashstore;
pub mod spscstate;
pub mod stdext;
pub mod store;
pub mod threadid;

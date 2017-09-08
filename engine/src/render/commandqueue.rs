#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;

/// Structure to store the render command queue abstraction.
///
/// No render command is evaluated immediatelly. they are collected in ques and
/// the execution is delayed until the process.
///
/// # TODO
/// Command queues are shared and can be filled from multiple threads,
/// but once sent for processing, no modification is allowed on them.
/// After process has finished the queue gets cleared and commands can be collected again.
pub struct CommandQueue {
    /// Stores the platform dependent implementation.
    pub platform: CommandQueueImpl,
}

impl CommandQueue {
    /// Creates a new queue to collect render commands.
    pub fn new() -> CommandQueue {
        CommandQueue { platform: CommandQueueImpl::new() }
    }
}



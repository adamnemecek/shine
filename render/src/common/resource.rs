use backend::*;

/// Trait for render command queue.
pub trait CommandQueue {
    fn add<C: Command>(&mut self, cmd: C);
}

/// Trait for render resources
pub trait Resource {
    /// Releases the allocated hw resources.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);
}




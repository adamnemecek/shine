use backend::*;

/// Trait for render command queue.
pub trait CommandQueue {
    fn add<C: Command>(&mut self, cmd: C);
}

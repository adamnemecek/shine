#![deny(missing_docs)]

/// Trait for render commands
pub trait Command: 'static {
    /// Return the sorting value for render command ordering
    fn get_sort_key(&self) -> usize;
}

/// Trait for command que.
pub trait CommandQueue {
    /// Type defining the render command
    type Command: Command;

    /// Store a command in the queue
    fn add(&self, c: Self::Command);
}

/// Trait for render resources
/// Cloning a Resource won't duplicate the resource, it is more like a handle, and only the handle
/// is duplicated. The same hw resource is referenced from both entity.
pub trait Resource: Clone {
    /// Command type used to store in the render command buffer.
    type Command: Command;

    /// Releases the allocated hw resources.
    fn release<Q: CommandQueue<Command=Self::Command>>(&self, queue: &Q);
}

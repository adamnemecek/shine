#![deny(missing_docs)]

use framework::*;

/// Trait for render resources
/// Cloning a Resource won't duplicate the resource, it is more like a handle, and only the handle
/// is duplicated. The same hw resource is referenced from both entity.
pub trait Resource<E: Engine>: Clone {
    /// Releases the allocated hw resources.
    fn release(&self, queue: &mut E::FrameCompose);
}

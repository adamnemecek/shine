#![deny(missing_docs)]

use framework::*;

/// Trait for nullable render resource
pub trait Handle {
    /// Creates a null handler in the same way as the default function in the Default trait
    fn null() -> Self;

    /// Returns if handle is valid. Note a valid handle does not mean it has an associated resources.
    /// A handle can be
    /// - null
    /// - valid but without hw resource
    /// - valid with allocated hw resources
    fn is_null(&self) -> bool;
}


/// Trait for render resources
/// Cloning a Resource won't duplicate the resource, it is more like a handle, and only the handle
/// is duplicated, thus both entity will reference the same hw resource.
pub trait Resource<E: Engine>: Handle + Clone {
    /// Creates a valid handle without allocating any hw resources.
    /// If self is not null a new handle is created as if a reset would have been called.
    fn create(&mut self, compose: &mut E::FrameCompose);

    /// Reset the handle without releasing the hw resources. If multiple handle point to the same
    /// resources, the other handles won't be effected.
    /// Note, resetting the last reference won't leak resources as Backend has some kind of
    /// reference counting and garbage collector to collect forgotten resources.
    /// See also the release function.
    fn reset(&mut self);

    /// Releases the allocated hw resources. If there are multiple handlers to the same
    /// resource, all the handle will be effected as the common resource is release.
    /// In other word, no handles are invalidated only the hw resources are released.
    /// See also the reset function.
    fn release(&self, queue: &mut E::FrameCompose);
}

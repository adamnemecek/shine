mod densestore;
mod hashstore;
mod store;
mod svector;
mod unitstore;
mod vectormask;

/// Trait implementations to iterate ower the stored data
mod dataiter;

pub use self::dataiter::*;
pub use self::densestore::*;
pub use self::hashstore::*;
pub use self::store::*;
pub use self::svector::*;
pub use self::unitstore::*;
pub use self::vectormask::*;

use svec::{DenseStore, HashStore, UnitStore};

pub type SDVector<T> = SVector<DenseStore<T>>;
pub fn new_dvec<T>() -> SDVector<T> {
    SVector::new(VectorMask::new(), DenseStore::new())
}

pub type SHVector<T> = SVector<HashStore<T>>;
pub fn new_hvec<T>() -> SHVector<T> {
    SVector::new(VectorMask::new(), HashStore::new())
}

pub type STVector = SVector<UnitStore>;
pub fn new_tvec() -> STVector {
    SVector::new(VectorMask::new(), UnitStore::new())
}

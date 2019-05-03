use crate::svec::{DenseStore, HashStore, SVector, UnitStore, VectorMask};

pub type SDVector<T> = SVector<DenseStore<T>>;
pub fn new_dvec<T>() -> SDVector<T> {
    SVector::new(VectorMask::new(), DenseStore::new())
}

pub type SSVector<T> = SVector<HashStore<T>>;
pub fn new_hvec<T>() -> SSVector<T> {
    SVector::new(VectorMask::new(), HashStore::new())
}

pub type STVector = SVector<UnitStore>;
pub fn new_tvec() -> STVector {
    SVector::new(VectorMask::new(), UnitStore::new())
}

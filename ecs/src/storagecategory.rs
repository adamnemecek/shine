pub trait StorageCategory {}

pub struct DenseStorage;
impl StorageCategory for DenseStorage {}

pub struct SparseStorage;
impl StorageCategory for SparseStorage {}

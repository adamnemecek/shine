use bitset::BitSetFast;
use svec::{SparseVector, SparseVectorStore};

pub struct SparseTVectorStore {
    unit: (),
}

impl SparseTVectorStore {
    pub fn new() -> Self {
        SparseTVectorStore { unit: () }
    }
}

impl SparseVectorStore for SparseTVectorStore {
    type Item = ();

    fn clear(&mut self) {}

    fn add(&mut self, _idx: usize, _value: Self::Item) {}

    fn remove(&mut self, _idx: usize) {}

    fn take(&mut self, _idx: usize) -> Self::Item {
        self.unit
    }

    fn replace(&mut self, _idx: usize, _value: Self::Item) -> Self::Item {
        self.unit
    }

    fn get(&self, _idx: usize) -> &Self::Item {
        &self.unit
    }

    fn get_mut(&mut self, _idx: usize) -> &mut Self::Item {
        &mut self.unit
    }
}

pub type SparseTVector = SparseVector<SparseTVectorStore>;

pub fn new_tvec() -> SparseTVector {
    SparseVector::new(BitSetFast::new(), SparseTVectorStore::new())
}

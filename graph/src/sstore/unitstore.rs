use sstore::SparseStore;

pub struct SparseUnitStore {
    unit: (),
}

impl SparseUnitStore {
    pub fn new() -> Self {
        SparseUnitStore { unit: () }
    }
}

impl SparseStore for SparseUnitStore {
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

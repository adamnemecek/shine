use smat::Store;

pub struct UnitStore {
    unit: (),
}

impl UnitStore {
    pub fn new() -> Self {
        UnitStore { unit: () }
    }
}

impl Store for UnitStore {
    type Item = ();

    fn clear(&mut self) {}

    fn insert(&mut self, _idx: usize, _value: Self::Item) {}

    fn remove(&mut self, _idx: usize) -> Self::Item {
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

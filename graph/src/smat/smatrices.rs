use crate::smat::{ArenaStore, CSMatrixMask, DenseStore, HCSMatrixMask, SMatrix, UnitStore};

pub type SDMatrix<T> = SMatrix<CSMatrixMask, DenseStore<T>>;
pub fn new_dmat<T>() -> SDMatrix<T> {
    SMatrix::new(CSMatrixMask::new(), DenseStore::new())
}

pub type SAMatrix<T> = SMatrix<CSMatrixMask, ArenaStore<T>>;
pub fn new_amat<T>() -> SAMatrix<T> {
    SMatrix::new(CSMatrixMask::new(), ArenaStore::new())
}

pub type STMatrix = SMatrix<CSMatrixMask, UnitStore>;
pub fn new_tmat() -> STMatrix {
    SMatrix::new(CSMatrixMask::new(), UnitStore::new())
}

pub type SHDMatrix<T> = SMatrix<HCSMatrixMask, DenseStore<T>>;
pub fn new_hdmat<T>() -> SHDMatrix<T> {
    SMatrix::new(HCSMatrixMask::new(), DenseStore::new())
}

pub type SHAMatrix<T> = SMatrix<HCSMatrixMask, ArenaStore<T>>;
pub fn new_hamat<T>() -> SHAMatrix<T> {
    SMatrix::new(HCSMatrixMask::new(), ArenaStore::new())
}

pub type SHTMatrix = SMatrix<HCSMatrixMask, UnitStore>;
pub fn new_htmat() -> SHTMatrix {
    SMatrix::new(HCSMatrixMask::new(), UnitStore::new())
}

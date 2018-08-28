mod arenastore;
mod csmatrixmask;
mod densestore;
mod hcsmatrixmask;
mod matrixmask;
mod smatrix;
mod store;
mod unitstore;

/// Trait implementations to iterate ower the stored data
mod dataiter;
/// Trait implementations to make an SVector joinable
mod join;

pub use self::arenastore::*;
pub use self::csmatrixmask::*;
pub use self::dataiter::*;
pub use self::densestore::*;
pub use self::hcsmatrixmask::*;
pub use self::join::*;
pub use self::matrixmask::*;
pub use self::smatrix::*;
pub use self::store::*;
pub use self::unitstore::*;

use smat::{ArenaStore, DenseStore, UnitStore};
use smat::{CSMatrixMask, HCSMatrixMask};

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

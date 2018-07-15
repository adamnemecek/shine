use std::collections::HashMap;

use bitset::BitSetFast;
use smat::SparseMatrixMask;

/// Compressed Sparse (Square) Row/Column matrix index handling of the non-zero items but no values.
/// Its a variant of the CSR data structure where a HashMap is
///  used to store the offset for the occupied rows(columns).
#[allow(dead_code)]
pub struct HCSMatrix {
    offset_mask: BitSetFast,
    offsets: HashMap<usize, usize>,
    indices: Vec<usize>,
}

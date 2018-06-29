use std::collections::HashMap;

use bitset::BitSetFast;
use smat::MatrixShape;

/// Compressed Sparse (Square) Row/Column matrix.
/// Its a varieóant of the CSR/C data structure where a HashMap is
///  used to store the offset for the occupied rows(columns).
#[allow(dead_code)]
pub struct HCSMatrix {
    shape: MatrixShape,
    nnz: usize,
    major_mask: BitSetFast,
    offsets: HashMap<usize, usize>,
    indices: Vec<usize>,
}

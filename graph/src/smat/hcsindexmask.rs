use std::collections::HashMap;

use smat::IndexMask;
use svec::BitMask;

/// Compressed Sparse (Square) Row/Column matrix index handling of the non-zero items but no values.
/// Its a variant of the CSR data structure where a HashMap is
///  used to store the offset for the occupied rows(columns).
#[allow(dead_code)]
pub struct HCSIndexMask {
    offset_mask: BitMask,
    offsets: HashMap<usize, usize>,
    indices: Vec<usize>,
}

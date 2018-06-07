extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;

use shine_graph::csmat::*;

#[test]
fn csr_mat() {
    let _ = env_logger::try_init();

    let mut csr = CSMat::new_row();
    assert_eq!(csr.nnz(), 0);

    let pos = csr.add(3, 4);
    assert_eq!(pos, InsertResult::New { pos: 0, size: 1 });
}

use shine_utils::stdext::SliceOrdExt;
use shine_testutils::init_test;

#[test]
fn lower_bound() {
    init_test(module_path!());

    let b: [u32; 0] = [];
    assert_eq!(b.lower_bound(&0), 0);

    let b = [1, 3, 3, 5];
    assert_eq!(b.lower_bound(&0), 0);
    assert_eq!(b.lower_bound(&1), 0);
    assert_eq!(b.lower_bound(&2), 1);
    assert_eq!(b.lower_bound(&3), 1);
    assert_eq!(b.lower_bound(&4), 3);
    assert_eq!(b.lower_bound(&5), 3);
    assert_eq!(b.lower_bound(&6), 4);

    let b = [1, 3, 3, 3, 3, 3, 5];
    assert_eq!(b.lower_bound(&0), 0);
    assert_eq!(b.lower_bound(&1), 0);
    assert_eq!(b.lower_bound(&2), 1);
    assert_eq!(b.lower_bound(&3), 1);
    assert_eq!(b.lower_bound(&4), 6);
    assert_eq!(b.lower_bound(&5), 6);
    assert_eq!(b.lower_bound(&6), 7);
}

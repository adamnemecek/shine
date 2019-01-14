use shine_ecs::EntityStore;
use shine_testutils::init_test;

#[test]
fn entity_create() {
    init_test(module_path!());

    let mut store = EntityStore::new();
    let e0 = store.create();
    let e1 = store.create();
    let e2 = store.create();
    assert_eq!(store.len(), 3);
    assert_eq!(e0.id(), 0);
    assert_eq!(e1.id(), 1);
    assert_eq!(e2.id(), 2);

    store.release(e1);
    assert_eq!(store.len(), 2);
    drop(store.drain_killed());
    assert_eq!(store.len(), 2);

    let e1 = store.create();
    assert_eq!(e1.id(), 1);
    drop(store.drain_raised());
    assert_eq!(store.len(), 3);

    let e3 = store.create();
    assert_eq!(e3.id(), 3);
    let _ = store.drain_raised();
    assert_eq!(store.len(), 4);

    store.release(e3);
    store.release(e1);
    store.release(e2);
    store.release(e0);
    let _ = store.drain_killed();
    assert_eq!(store.len(), 0);

    let e0 = store.create();
    let e1 = store.create();
    let e2 = store.create();
    assert_eq!(e0.id(), 0);
    assert_eq!(e1.id(), 1);
    assert_eq!(e2.id(), 2);
    let _ = store.drain_raised();
    assert_eq!(store.len(), 3);
}

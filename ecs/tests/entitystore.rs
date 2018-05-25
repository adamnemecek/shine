extern crate shine_ecs as ecs;
extern crate env_logger;

use ecs::*;

#[test]
fn entity_create()
{
    let _ = env_logger::try_init();

    let mut store = EntityStore::new();
    let e0 = store.create();
    let e1 = store.create();
    let e2 = store.create();
    assert!(store.len() == 3);
    assert!(e0.id() == 0);
    assert!(e1.id() == 1);
    assert!(e2.id() == 2);

    store.release(e1);
    assert!(store.len() == 2);
    drop(store.drain_killed());
    assert!(store.len() == 2);

    let e1 = store.create();
    assert!(e1.id() == 1);
    drop(store.drain_raised());
    assert!(store.len() == 3);

    let e3 = store.create();
    assert!(e3.id() == 3);
    let _ = store.drain_raised();
    assert!(store.len() == 4);

    store.release(e3);
    store.release(e1);
    store.release(e2);
    store.release(e0);
    let _ = store.drain_killed();
    assert!(store.len() == 0);

    let e0 = store.create();
    let e1 = store.create();
    let e2 = store.create();
    assert!(e0.id() == 0);
    assert!(e1.id() == 1);
    assert!(e2.id() == 2);
    let _ = store.drain_raised();
    assert!(store.len() == 3);
}


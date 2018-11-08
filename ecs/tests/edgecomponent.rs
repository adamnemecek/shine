extern crate log;
extern crate shine_ecs as ecs;
extern crate shine_testutils;

use ecs::{DenseStorage, Edge, EdgeComponent, Entity, EntityComponent, IntoJoinExt, SparseStorage, World};
use log::{debug, trace};
use shine_testutils::init_test;

#[derive(Debug, PartialEq)]
struct Force {
    x: i32,
    y: i32,
    z: i32,
}
impl EntityComponent for Force {
    type StorageCategory = DenseStorage;
}

#[derive(Debug, PartialEq)]
struct Acceleration {
    x: i32,
    y: i32,
    z: i32,
}
impl EntityComponent for Acceleration {
    type StorageCategory = DenseStorage;
}

#[derive(Debug)]
struct Weight {
    w: i32,
}
impl EdgeComponent for Weight {
    type StorageCategory = SparseStorage;
}

#[test]
fn test_component() {
    init_test(module_path!());

    let mut world = World::new();

    world.register_entity::<Force>();
    world.register_entity::<Acceleration>();
    world.register_edge::<Weight>();

    debug!("create instances");
    {
        let mut ent = world.entities_mut();
        let mut force = world.get_entity_mut::<Force>();
        let mut weight = world.get_edge_mut::<Weight>();

        for i in 0..30 {
            let e = ent.create();
            force.add(e, Force { x: i, y: 2 * i, z: 0 });
        }

        weight.add(Edge::from_ids(1, 2), Weight { w: 1 });
        weight.add(Edge::from_ids(1, 3), Weight { w: 2 });
        weight.add(Edge::from_ids(2, 3), Weight { w: 3 });
        weight.add(Edge::from_ids(4, 5), Weight { w: 4 });
    }

    debug!("update instances");
    {
        let force = world.get_entity::<Force>();
        let weight = world.get_edge::<Weight>();
        let mut acc = world.get_entity_mut::<Acceleration>();

        (force.read(), weight.read()).join_all(|source, (f, w)| {
            trace!("source: {:?} = {:?}", source, f);
            (acc.write(), w).join_all(|target, (mut a, w)| {
                trace!("tagret {:?} update {:?} += {:?} * {:?} ", target, a, f, w);
                let a = a.get_or(Acceleration { x: 0, y: 0, z: 0 });
                a.x += f.x * w.w;
                a.y += f.y * w.w;
                a.z += f.z * w.w;
                trace!("target {:?} result {:?}", target, a);
            })
        });
    }

    debug!("get");
    {
        let acc = world.get_entity::<Acceleration>();
        assert_eq!(acc.count(), 3);
        assert_eq!(acc.get(Entity::from_id(2)), Some(&Acceleration { x: 1, y: 2, z: 0 }));
        assert_eq!(acc.get(Entity::from_id(3)), Some(&Acceleration { x: 8, y: 16, z: 0 }));
        assert_eq!(acc.get(Entity::from_id(5)), Some(&Acceleration { x: 16, y: 32, z: 0 }));
    }
}

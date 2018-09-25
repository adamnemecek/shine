#[macro_use]
extern crate log;
extern crate shine_ecs as ecs;
extern crate shine_testutils;

use ecs::*;
use shine_testutils::*;

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
    init_test_logger(module_path!());

    let mut world = World::new();

    world.register_entity::<Force>();
    world.register_entity::<Acceleration>();
    world.register_edge::<Weight>();

    trace!("create instances");
    {
        let mut ent = world.entities_mut();
        let mut force = world.get_entity_mut::<Force>();
        let mut weight = world.get_edge_mut::<Weight>();

        for i in 0..30 {
            let e = ent.create();
            force.add(e, Force { x: i, y: 2 * i, z: 0 });
        }

        weight.add(Edge::from_ids(0, 2), Weight { w: 1 });
        weight.add(Edge::from_ids(0, 3), Weight { w: 2 });
        weight.add(Edge::from_ids(4, 5), Weight { w: 3 });
    }

    trace!("update instances");
    {
        let force = world.get_entity::<Force>();
        let weight = world.get_edge::<Weight>();
        let mut acc = world.get_entity_mut::<Acceleration>();

        (force.read(), weight.read()).join_all(|_source, (f, w)| {
            (acc.write(), w).join_all(|_target, (mut a, w)| {
                let a = a.get_or(Acceleration { x: 0, y: 0, z: 0 });
                a.x += f.x * w.w;
                a.y += f.y * w.w;
                a.z += f.z * w.w;
            })
        });
    }
}

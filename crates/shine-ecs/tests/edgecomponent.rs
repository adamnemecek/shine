#![feature(custom_attribute)]

use log::{debug, trace};
use shine_ecs::entities::{ds, es, Edge, Entity, IntoJoinExt};
use shine_ecs::world::{EntityWorld, World};
use shine_testutils::init_test;

#[derive(Debug, PartialEq)]
struct Force {
    x: i32,
    y: i32,
    z: i32,
}
impl es::Component for Force {
    type Store = es::DenseStore<Self>;
}

#[derive(Debug, PartialEq)]
//#[derive(EntityComponent<DenseStore>)]
struct Acceleration {
    x: i32,
    y: i32,
    z: i32,
}
impl es::Component for Acceleration {
    type Store = es::DenseStore<Self>;
}

#[derive(Debug)]
struct Weight {
    w: i32,
}
impl ds::Component for Weight {
    type Mask = ds::CSMatrixMask;
    type Store = ds::ArenaStore<Self>;
}

#[test]
#[ignore]
fn test_component() {
    init_test(module_path!());

    let mut world = World::new();

    world.register_entity_component::<Force>();
    world.register_entity_component::<Acceleration>();
    world.register_edge_component::<Weight>();

    debug!("create instances");
    {
        let mut ent = world.entities_mut();
        let mut force = world.entity_components_mut::<Force>();
        let mut weight = world.edge_components_mut::<Weight>();

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
        let force = world.entity_components::<Force>();
        let weight = world.edge_components::<Weight>();
        let mut acc = world.entity_components_mut::<Acceleration>();

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
        let acc = world.entity_components::<Acceleration>();
        assert_eq!(acc.count(), 3);
        assert_eq!(acc.get(Entity::from_id(2)), Some(&Acceleration { x: 1, y: 2, z: 0 }));
        assert_eq!(acc.get(Entity::from_id(3)), Some(&Acceleration { x: 8, y: 16, z: 0 }));
        assert_eq!(acc.get(Entity::from_id(5)), Some(&Acceleration { x: 16, y: 32, z: 0 }));
    }
}

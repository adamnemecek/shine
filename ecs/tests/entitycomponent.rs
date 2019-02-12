use log::{debug, trace};
use shine_ecs::entities::{storage, Entity, EntityComponent, IntoJoinExt};
use shine_ecs::{EntityWorld, World};
use shine_testutils::init_test;

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}
impl EntityComponent for Pos {
    type StorageCategory = storage::Dense;
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}
impl EntityComponent for Velocity {
    type StorageCategory = storage::Sparse;
}

#[test]
fn test_component() {
    init_test(module_path!());

    let mut world = World::new();

    world.register_entity_component::<Pos>();
    world.register_entity_component::<Velocity>();

    debug!("create instances");
    {
        let mut ent = world.entities_mut();
        let mut pos = world.get_entity_component_mut::<Pos>();
        let mut vel = world.get_entity_component_mut::<Velocity>();

        for i in 0..30 {
            let e = ent.create();
            pos.add(e, Pos { x: i, y: 2 * i, z: 0 });

            if i % 2 == 0 {
                vel.add(e, Velocity { x: 0, y: 0, z: 3 * i });
            }
        }
    }

    debug!("update instances");
    {
        let mut pos = world.get_entity_component_mut::<Pos>();
        let vel = world.get_entity_component::<Velocity>();

        (pos.update(), vel.read()).join_all(|id, (p, v)| {
            trace!("{:?}: {:?} {:?}", id, p, v);
            p.x += v.x;
            p.y += v.y;
            p.z += v.z;
        });
    }

    debug!("get");
    {
        let mut pos = world.get_entity_component_mut::<Pos>();
        assert_eq!(pos.get_entry(Entity::from_id(2)).remove(), Some(Pos { x: 2, y: 4, z: 6 }));
        assert_eq!(pos.get(Entity::from_id(1)), Some(&Pos { x: 1, y: 2, z: 0 }));
        assert_eq!(pos.remove(Entity::from_id(4)), Some(Pos { x: 4, y: 8, z: 12 }));
    }
}

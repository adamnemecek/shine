#[macro_use]
extern crate log;
extern crate shine_ecs as ecs;
extern crate shine_testutils;

use ecs::*;
use shine_testutils::*;

#[derive(Debug)]
struct Pos {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Pos {
    type StorageCategory = DenseStorage;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Velocity {
    type StorageCategory = SparseStorage;
}

#[test]
fn test_component() {
    init_test_logger(module_path!());

    let mut world = World::new();

    world.register_component::<Pos>();
    world.register_component::<Velocity>();

    trace!("create instances");
    {
        let mut ent = world.entities_mut();
        let mut pos = world.components_mut::<Pos>();
        let mut vel = world.components_mut::<Velocity>();

        for i in 0..30 {
            let e = ent.create();
            pos.add(
                e,
                Pos {
                    x: i as f32,
                    y: (2 * i) as f32,
                    z: 0.,
                },
            );

            if i % 2 == 0 {
                vel.add(
                    e,
                    Velocity {
                        x: 0.,
                        y: 0.,
                        z: (3 * i) as f32,
                    },
                );
            }
        }
    }

    trace!("update instances");
    {
        /*let mut pos = world.components::<Pos>();
        let mut vel = world.components_mut::<Velocity>();

        (pos.store.write(), vel.store.read()).join_all(|id, (p, v)| {
            *p.x += v.x;
            *p.y += v.y;
            *p.z += v.z;
        });*/
    }
}

extern crate shine_ecs as ecs;
extern crate env_logger;
extern crate shred;
extern crate hibitset;

use ecs::*;


#[derive(Debug)]
struct Pos {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Pos {
    type Storage = DenseStorage<Pos>;
}

impl Component for Velocity {
    type Storage = DenseStorage<Velocity>;
}


#[test]
fn world_simple()
{
    let _ = env_logger::try_init();

    let mut world = World::new();

    world.register_component::<Pos>();
    world.register_component::<Velocity>();

    {
        let mut ent = world.resources.fetch_mut::<EntityStore>();
        let mut pos = world.resources.fetch_mut::<<Pos as Component>::Storage>();
        let mut vel = world.resources.fetch_mut::<<Velocity as Component>::Storage>();

        for i in 0..30 {
            let e = ent.create();
            if i % 3 == 0 {
                pos.insert(e, Pos { x: 0., y: 0., z: 0. });
            }
            if i % 2 == 0 {
                vel.insert(e, Velocity { x: 0., y: 0., z: 1. });
            }


            for p in pos.iter_mut() {
                p.x += 0.1;
                p.y += 0.1;
                p.z += 0.1;
            }
        }
    }

    {
        let (_e, _p, mut _v): (ReadEntities, ReadComponent<Pos>, WriteComponent<Velocity>) = world.system_data();
    }
}


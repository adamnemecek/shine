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

#[derive(Debug)]
struct Spring {
    w: f32,
}

impl Component for Pos {
    type Storage = DenseStorage<Pos>;
}

impl Component for Velocity {
    type Storage = DenseStorage<Velocity>;
}

impl Link for Spring {
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
        let mut spring = world.resources.fetch_mut::<<Spring as Link>::Storage>();

        for i in 0..30 {
            let e = ent.create();
            if i % 3 == 0 {
                pos.insert(e, Pos { x: 0., y: 0., z: 0. });
                pos[e].x = 1.;
            }
            if i % 2 == 0 {
                vel.insert(e, Velocity { x: 0., y: 0., z: 1. });
            }


            {
                let mut iter = join_wr(&mut pos, &vel);
                while let Some(p) = iter.next() {
                    println!("e:{:?}, p:{:?}, v: {:?}", p.0, p.1, p.2);
                    p.1.x += p.2.x;
                    p.1.y += p.2.y;
                    p.1.z += p.2.z;
                }
            }

            {
                let mut iter = join_wr(&mut pos, spring.backward_view());
                while let Some(p) = iter.next() {
                    println!("e:{:?}, p:{:?}, v: {:?}", p.0, p.1, p.2);
                    p.1.x += p.2.w;
                    p.1.y += p.2.w;
                    p.1.z += p.2.w;
                }
            }

            {
                let mut iter = join_qrw(&mut pos, &vel);
                while let Some(p) = iter.next() {
                    match (p.1, p.2) {
                        (Some(p), Some(v)) => p.x += v.x,
                        (None, Some(v)) => iter.create(Pos { x: v.x, y: v.y, z: v.z }),
                        _ => {}
                    }
                }
            }
        }

        {
            let (_e, _p, mut _v): (ReadEntities, ReadComponent<Pos>, WriteComponent<Velocity>) = world.system_data();
        }
    }
}

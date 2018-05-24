extern crate shine_ecs as ecs;
extern crate env_logger;
extern crate shred;

use ecs::*;


struct Pos {
    x: f32,
    y: f32,
    z: f32,
}

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

        for _i in 0..10 {
            let e = ent.create();
            pos.insert(e, Pos { x: 0., y: 0., z: 0. });
            vel.insert(e, Velocity { x: 0., y: 0., z: 1. });
        }
    }

    {
        let (e, p, mut v): (ReadEntites, ReadComponent<Pos>, WriteComponent<Velocity>) = world.system_data();
        //println!("e {:?}", e.get_mask());
        println!("p {:?}", p.get_mask());
        println!("v {:?}", v.get_mask_mut());
    }

//world.resources.sys



    /*world.exec(|(p,v) : (Pos, Velocity)| {

    });*/
}


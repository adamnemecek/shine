use shine_ecs::{ResourceWorld, World};
use shine_testutils::init_test;
use shred::{DispatcherBuilder, Read, System, Write};

#[derive(Debug)]
struct ResA {
    value: i32,
}

#[derive(Debug)]
struct ResB {
    value: i32,
}

struct PrintSystem;
impl<'a> System<'a> for PrintSystem {
    type SystemData = (Option<Read<'a, ResA>>, Option<Write<'a, ResB>>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, b) = data;
        let a = a.unwrap();
        let mut b = b.unwrap();

        b.value += a.value;

        log::trace!("{:?}", &*a);
        log::trace!("{:?}", &*b);
    }
}

#[test]
fn test_system() {
    init_test(module_path!());

    let mut world = World::new();
    world.register_resource_with(ResA { value: 11 });
    world.register_resource_with(ResB { value: 1 });

    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();

    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);

    assert_eq!(world.resource::<ResA>().value, 11);
    assert_eq!(world.resource::<ResB>().value, 56);
}

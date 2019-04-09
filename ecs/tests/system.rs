use shine_ecs::{DispatcherBuilder, ResourceWorld, Scoped, ScopedReadOptional, ScopedSystem, ScopedWriteOptional, System, World};
use shine_testutils::init_test;

struct TestScope;
struct LogicScopes;

#[derive(Debug, Default)]
struct ResA {
    value: i32,
}
impl Scoped for ResA {
    type Scope = TestScope;
}

#[derive(Debug)]
struct ResB {
    value: i32,
}
impl Scoped for ResB {
    type Scope = TestScope;
}

struct PrintSystem;
impl<'a> System<'a> for PrintSystem {
    type SystemData = (ScopedReadOptional<'a, ResA>, ScopedWriteOptional<'a, ResB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, b) = data;
        let a = &*a;
        let b = &*b;
        let a = a.unwrap();
        let mut b = b.unwrap();

        b.value += a.value;

        log::trace!("{:?}", &*a);
        log::trace!("{:?}", &*b);
    }
}
impl<'a> ScopedSystem<'a> for PrintSystem {
    type Scope = TestScope;
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

    world.dispatch::<TestScope>(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);
    world.dispatch(&mut dispatcher);

    assert_eq!(world.resource::<ResA>().value, 11);
    assert_eq!(world.resource::<ResB>().value, 56);
}

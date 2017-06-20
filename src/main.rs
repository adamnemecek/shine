mod container;

use container::spscstate::SPSCState;

type SPSCInt = SPSCState<i32>;

fn main() {
    let state = SPSCInt::new();

    let mut p = state.produce();
    let c = state.consume();

    *p = 123;
    println!("{}", *c);

    println!("Hello, world!");
}

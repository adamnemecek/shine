mod container;

use container::spscstate::SPSCState;

type SPSCInt = SPSCState<i32>;

fn main() {
    let state = SPSCInt::new();

    for x in 0..500 {
        {
            let mut p = state.produce();
            *p = x;
        }

        {
            let c = state.consume();
            assert_eq!(*c, x);
        }
    }
}

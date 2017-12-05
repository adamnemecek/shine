mod factory;
mod store;

pub use self::store::*;
pub use self::factory::*;

pub fn create<F: Factory>(f: F) -> Store<F> {
    Store::new(f)
}
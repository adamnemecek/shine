mod factory;
mod store;

pub use self::store::*;
pub use self::factory::*;

pub fn create<F: Factory>(f: F) -> HashStore<F> {
    HashStore::new(f)
}
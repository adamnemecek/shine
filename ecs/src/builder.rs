use crate::Scoped;
use shred;
use std::marker::PhantomData;

/// System that requires a logic scope.
pub trait ScopedSystem<'a>: shred::System<'a>
where
    Self::SystemData: shred::DynamicSystemData<'a> + Scoped,
{
    type Scope;
}

/// shred::Dispatcher extended with a logic scope
pub struct Dispatcher<'a, 'b, S> {
    pub(crate) inner: shred::Dispatcher<'a, 'b>,
    ph: PhantomData<S>,
}

pub struct DispatcherBuilder<'a, 'b, S> {
    inner: shred::DispatcherBuilder<'a, 'b>,
    ph: PhantomData<S>,
}

impl<'a, 'b, S> DispatcherBuilder<'a, 'b, S> {
    pub fn new() -> DispatcherBuilder<'a, 'b, S> {
        DispatcherBuilder {
            inner: Default::default(),
            ph: PhantomData,
        }
    }

    pub fn with<T>(mut self, system: T, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> ScopedSystem<'c, Scope = S> + Send + 'a,
    {
        self.inner.add(system, name, dep);
        self
    }

    pub fn add<T>(&mut self, system: T, name: &str, dep: &[&str])
    where
        T: for<'c> ScopedSystem<'c, Scope = S> + Send + 'a,
    {
        self.inner.add(system, name, dep);
    }

    pub fn with_barrier(mut self) -> Self {
        self.inner.add_barrier();
        self
    }

    pub fn add_barrier(&mut self) {
        self.inner.add_barrier();
    }

    #[cfg(feature = "parallel")]
    pub fn with_pool(mut self, pool: ::std::sync::Arc<::rayon::ThreadPool>) -> Self {
        self.inner.with_pool(pool);
        self
    }

    #[cfg(feature = "parallel")]
    pub fn add_pool(&mut self, pool: ::std::sync::Arc<::rayon::ThreadPool>) {
        self.inner.add_pool(pool);
    }

    pub fn print_par_seq(&self) {
        self.inner.print_par_seq();
    }

    pub fn build(self) -> Dispatcher<'a, 'b, S> {
        let inner = self.inner.build();
        Dispatcher { inner, ph: PhantomData }
    }
}

/*#[cfg(feature = "parallel")]
impl<'b, S> DispatcherBuilder<'static, 'b, S> {
    pub fn build_async<R>(self, world: R,
    ) -> AsyncDispatcher<'b, R, S> {
        let d = self.inner.build_async(world);
        AsyncDispatcher(d, PhantomData)
    }
}*/

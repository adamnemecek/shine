use shred::{Fetch, FetchMut, Read, Resource, ResourceId, Resources, SystemData, Write};
use std::ops::{Deref, DerefMut};

/// Trait to define scope for resources
pub trait Scoped {
    type Scope;
}

macro_rules! impl_scoped {
    ( $($ty:ident),* ) => {
        impl<SC,$($ty),*> Scoped for ( $( $ty , )* )
            where $( $ty : Scoped<Scope=SC> ),*
            {
                type Scope = SC;
            }
    };
}

mod impl_scoped {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;

    impl_scoped!(A);
    impl_scoped!(A, B);
    impl_scoped!(A, B, C);
    impl_scoped!(A, B, C, D);
    impl_scoped!(A, B, C, D, E);
    impl_scoped!(A, B, C, D, E, F);
    impl_scoped!(A, B, C, D, E, F, G);
    impl_scoped!(A, B, C, D, E, F, G, H);
    impl_scoped!(A, B, C, D, E, F, G, H, I);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_scoped!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}

pub struct ScopedRead<'a, T>
where
    T: Resource + Scoped,
{
    inner: Read<'a, T>,
}

impl<'a, T> Scoped for ScopedRead<'a, T>
where
    T: Resource + Scoped,
{
    type Scope = <T as Scoped>::Scope;
}

impl<'a, T> Deref for ScopedRead<'a, T>
where
    T: Resource + Scoped,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'a, T> From<Fetch<'a, T>> for ScopedRead<'a, T>
where
    T: Resource + Scoped,
{
    fn from(inner: Fetch<'a, T>) -> Self {
        ScopedRead { inner: inner.into() }
    }
}

impl<'a, T> SystemData<'a> for ScopedRead<'a, T>
where
    T: Resource + Scoped,
{
    fn setup(_world: &mut Resources) {}

    fn fetch(world: &'a Resources) -> Self {
        world.fetch::<T>().into()
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

pub struct ScopedReadOptional<'a, T>
where
    T: Resource + Scoped,
{
    inner: Option<ScopedRead<'a, T>>,
}

impl<'a, T> Scoped for ScopedReadOptional<'a, T>
where
    T: Resource + Scoped,
{
    type Scope = <T as Scoped>::Scope;
}

impl<'a, T> Deref for ScopedReadOptional<'a, T>
where
    T: Resource + Scoped,
{
    type Target = Option<ScopedRead<'a, T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> SystemData<'a> for ScopedReadOptional<'a, T>
where
    T: Resource + Scoped,
{
    fn setup(_world: &mut Resources) {}

    fn fetch(world: &'a Resources) -> Self {
        ScopedReadOptional {
            inner: world.try_fetch().map(Into::into),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

pub struct ScopedWrite<'a, D>
where
    D: Resource + Scoped,
{
    inner: Write<'a, D>,
}

impl<'a, D> Scoped for ScopedWrite<'a, D>
where
    D: Resource + Scoped,
{
    type Scope = <D as Scoped>::Scope;
}

impl<'a, T> Deref for ScopedWrite<'a, T>
where
    T: Resource + Scoped,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'a, T> DerefMut for ScopedWrite<'a, T>
where
    T: Resource + Scoped,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl<'a, T> From<FetchMut<'a, T>> for ScopedWrite<'a, T>
where
    T: Resource + Scoped,
{
    fn from(inner: FetchMut<'a, T>) -> Self {
        ScopedWrite { inner: inner.into() }
    }
}

impl<'a, T> SystemData<'a> for ScopedWrite<'a, T>
where
    T: Resource + Scoped,
{
    fn setup(_world: &mut Resources) {}

    fn fetch(world: &'a Resources) -> Self {
        world.fetch_mut::<T>().into()
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }
}

pub struct ScopedWriteOptional<'a, T>
where
    T: Resource + Scoped,
{
    inner: Option<ScopedWrite<'a, T>>,
}

impl<'a, T> Scoped for ScopedWriteOptional<'a, T>
where
    T: Resource + Scoped,
{
    type Scope = <T as Scoped>::Scope;
}

impl<'a, T> Deref for ScopedWriteOptional<'a, T>
where
    T: Resource + Scoped,
{
    type Target = Option<ScopedWrite<'a, T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for ScopedWriteOptional<'a, T>
where
    T: Resource + Scoped,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T> SystemData<'a> for ScopedWriteOptional<'a, T>
where
    T: Resource + Scoped,
{
    fn setup(_world: &mut Resources) {}

    fn fetch(world: &'a Resources) -> Self {
        ScopedWriteOptional {
            inner: world.try_fetch_mut().map(Into::into),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }
}
/*
impl<'a, T> SystemData<'a> for Option<ScopedWrite<'a, T>>
where
    T: Resource + Scoped,
{
    fn setup(_world: &mut Resources) {}

    fn fetch(world: &'a Resources) -> Self {
        ScopedWriteOptional {
            inner: world.try_fetch_mut().map(Into::into),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<T>()]
    }
}
*/

use std::ops::{Deref, DerefMut};
use shred::{Resources, ResourceId, Read, Write, SystemData};

use componentcontainer::ComponentContainer;

pub trait Component {
    type Storage: 'static + ComponentContainer;
}


/// Grant read access for a component
pub struct ReadComponent<'a, C: Component> {
    inner: Read<'a, C::Storage>,
}

impl<'a, C: Component> Deref for ReadComponent<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: Component> SystemData<'a> for ReadComponent<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadComponent { inner: res.fetch::<C::Storage>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<C::Storage>(),
        ]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}


/// Grant read/write access to a component
pub struct WriteComponent<'a, C: Component> {
    inner: Write<'a, C::Storage>,
}

impl<'a, C: Component> Deref for WriteComponent<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: Component> DerefMut for WriteComponent<'a, C> {
    fn deref_mut(&mut self) -> &mut C::Storage {
        self.inner.deref_mut()
    }
}

impl<'a, C: Component> SystemData<'a> for WriteComponent<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteComponent { inner: res.fetch_mut::<C::Storage>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<C::Storage>(),
        ]
    }
}

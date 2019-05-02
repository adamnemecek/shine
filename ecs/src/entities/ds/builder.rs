use crate::entities::{ds, Edge};
use crate::world::EntityWorld;

pub struct Builder<'a, W>
where
    W: EntityWorld,
{
    edge: Edge,
    world: &'a mut W,
}

impl<'a, W> Builder<'a, W>
where
    W: EntityWorld,
{
    pub fn new(world: &mut W, edge: Edge) -> Builder<'_, W> {
        Builder { edge, world }
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: 'static + ds::Component,
    {
        {
            let mut store = self.world.edge_components_mut::<T>();
            store.add(self.edge, component);
        }
        self
    }
}

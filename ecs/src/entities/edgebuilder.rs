use crate::entities::{Edge, EdgeComponent};
use crate::EntityWorld;

pub struct EdgeBuilder<'a, W>
where
    W: EntityWorld,
{
    edge: Edge,
    world: &'a mut W,
}

impl<'a, W> EdgeBuilder<'a, W>
where
    W: EntityWorld,
{
    pub fn new(world: &mut W, edge: Edge) -> EdgeBuilder<'_, W> {
        EdgeBuilder { edge, world }
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: 'static + EdgeComponent,
    {
        {
            let mut store = self.world.edge_components_mut::<T>();
            store.add(self.edge, component);
        }
        self
    }
}

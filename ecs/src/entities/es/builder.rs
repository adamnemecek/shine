use crate::entities::{es, Entity};
use crate::world::EntityWorld;

pub struct Builder<'a, W>
where
    W: EntityWorld,
{
    entity: Entity,
    world: &'a mut W,
}

impl<'a, W> Builder<'a, W>
where
    W: EntityWorld,
{
    pub fn new(world: &mut W) -> Builder<'_, W> {
        let entity = world.entities_mut().create();
        Builder { world, entity }
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: 'static + es::Component,
    {
        {
            let mut store = self.world.entity_components_mut::<T>();
            store.add(self.entity, component);
        }
        self
    }
}

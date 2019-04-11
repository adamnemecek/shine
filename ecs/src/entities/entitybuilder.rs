use crate::entities::{Entity, EntityComponent};
use crate::world::EntityWorld;

pub struct EntityBuilder<'a, W>
where
    W: EntityWorld,
{
    entity: Entity,
    world: &'a mut W,
}

impl<'a, W> EntityBuilder<'a, W>
where
    W: EntityWorld,
{
    pub fn new(world: &mut W) -> EntityBuilder<'_, W> {
        let entity = world.entities_mut().create();
        EntityBuilder { world, entity }
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: 'static + EntityComponent,
    {
        {
            let mut store = self.world.entity_components_mut::<T>();
            store.add(self.entity, component);
        }
        self
    }
}

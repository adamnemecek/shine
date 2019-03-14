use crate::entities::{Entity, EntityComponent};
use crate::EntityWorld;

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
    pub fn new<'b>(world: &'b mut W) -> EntityBuilder<'b, W> {
        let entity = world.entities_mut().create();
        EntityBuilder { world, entity }
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: 'static + EntityComponent,
    {
        {
            let mut store = self.world.get_entity_component_mut::<T>();
            store.add(self.entity, component);
        }
        self
    }
}

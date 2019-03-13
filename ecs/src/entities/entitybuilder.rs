use crate::entities::{storage, Entity, EntityComponent, EntityComponentDescriptor, EntityComponentStore};
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

    /*pub fn with<S, T>(&mut self, component: T) -> &mut Self
    where
        S: storage::Category,
        T: 'static + EntityComponent<StorageCategory = S>,
        (S, T): EntityComponentDescriptor,
    {
        {
            let mut store = self.world.get_entity_component_mut::<T>();
            store.add(self.entity, component);
        }
        self
    }*/
}

use componentcontainer::ComponentContainer;

pub trait Component {
    type Storage: 'static + ComponentContainer;
}

/*
pub trait ReadComponentStorage {
    /// The component type of the storage
    type Component: Component;

    /// Get immutable to an `Entity`s component
    fn get(&self, entity: Entity) -> Option<&Self::Component>;

    //fn iter(&self) -> Iterator<(u32, &Self::Component)>;
}

pub trait WriteComponentStorage: ReadComponentStorage {
    /// Get mutable access to an `Entity`s component
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Component>;

    /// Insert a component for an `Entity`
    fn insert(&mut self, entity: Entity, comp: Self::Component);

    /// Remove the component for an `Entity`
    fn remove(&mut self, entity: Entity);
}
*/


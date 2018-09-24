use component::{Component, ComponentDescriptor, ComponentStore};
use entity::EntityStore;
use shred::{Fetch, FetchMut, Resources, SystemData};

pub struct World {
    pub resources: Resources,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            resources: Resources::new(),
        };

        world.resources.insert(EntityStore::new());

        world
    }

    pub fn entities<'a>(&'a self) -> Fetch<'a, EntityStore> {
        self.resources.fetch()
    }

    pub fn entities_mut<'a>(&'a self) -> FetchMut<'a, EntityStore> {
        self.resources.fetch_mut()
    }

    pub fn register_component<C: ComponentDescriptor>(&mut self) {
        self.resources.insert::<ComponentStore<C>>(Default::default());
    }

    pub fn components<'a, C: ComponentDescriptor>(&'a self) -> Fetch<'a, ComponentStore<C>> {
        self.resources.fetch()
    }

    pub fn components_mut<'a, C: ComponentDescriptor>(&'a self) -> FetchMut<'a, ComponentStore<C>> {
        self.resources.fetch_mut()
    }

    /*pub fn register_link<L>(&mut self)
        where
            L: LinkStore,
            L::Store: Default
    {
        self.resources.insert::<L::Store>(Default::default());
    }

    pub fn links<'a, T: LinkStore>(&'a self) -> Fetch<'a, <T as LinkStore>::Store> {
        self.resources.fetch()
    }

    pub fn links_mut<'a, T: LinkStore>(&'a self) -> FetchMut<'a, <T as LinkStore>::Store> {
        self.resources.fetch_mut()
    }*/

    pub fn system_data<'a, T: SystemData<'a>>(&'a self) -> T {
        SystemData::fetch(&self.resources)
    }
}

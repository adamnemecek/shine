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

    /* pub fn register_component<C>(&mut self)
        where
            C: ComponentStore,
            C::Storage: Default
    {
        self.resources.insert::<C::Storage>(Default::default());
    }

    pub fn register_link<L>(&mut self)
        where
            L: LinkStore,
            L::Storage: Default
    {
        self.resources.insert::<L::Storage>(Default::default());
    }

    pub fn entities<'a>(&'a self) -> Fetch<'a, EntityStore> {
        self.resources.fetch()
    }

    pub fn entities_mut<'a>(&'a self) -> FetchMut<'a, EntityStore> {
        self.resources.fetch_mut()
    }

    pub fn components<'a, T: ComponentStore>(&'a self) -> Fetch<'a, <T as ComponentStore>::Storage> {
        self.resources.fetch()
    }

    pub fn components_mut<'a, T: ComponentStore>(&'a self) -> FetchMut<'a, <T as ComponentStore>::Storage> {
        self.resources.fetch_mut()
    }

    pub fn links<'a, T: LinkStore>(&'a self) -> Fetch<'a, <T as LinkStore>::Storage> {
        self.resources.fetch()
    }

    pub fn links_mut<'a, T: LinkStore>(&'a self) -> FetchMut<'a, <T as LinkStore>::Storage> {
        self.resources.fetch_mut()
    }*/

    pub fn system_data<'a, T: SystemData<'a>>(&'a self) -> T {
        SystemData::fetch(&self.resources)
    }
}

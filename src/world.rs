use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

pub struct World {}

impl World {
    fn new() -> World {
        World {}
    }
}


#[derive(Clone)]
pub struct WorldWrapper(pub Rc<RefCell<World>>);

impl WorldWrapper {
    pub fn new() -> WorldWrapper {
        WorldWrapper(Rc::new(RefCell::new(World::new())))
    }
}

impl Deref for WorldWrapper {
    type Target = RefCell<World>;

    fn deref(&self) -> &RefCell<World> {
        self.0.deref()
    }
}
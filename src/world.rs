use std::rc::Rc;
use std::cell::RefCell;

use view::View;
use render::*;

pub struct WorldData {
    t: f32,
}

impl WorldData {
    fn new() -> WorldData {
        WorldData {
            t: 0.0,
        }
    }
}


pub struct World(Rc<RefCell<WorldData>>);

impl World {
    pub fn new() -> World {
        World(Rc::new(RefCell::new(WorldData::new())))
    }

    pub fn create_view(&mut self, window: &mut Window) -> View {
        View::new(World(self.0.clone()), window)
    }

    pub fn update(&mut self) {
        let mut world = self.0.borrow_mut();
        world.t += 0.01;
    }

    pub fn get_t(&self) -> f32 {
        let world = self.0.borrow();
        world.t
    }
}

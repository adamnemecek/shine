use std::rc::Rc;
use std::cell::RefCell;

use world::*;

pub struct GameData {
    pub image_store: ImageStore,
    pub t: f32,
}

impl GameData {
    fn new() -> GameData {
        GameData {
            image_store: imagestore::create(),
            t: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.image_store.update();
        self.t += 0.01;
    }
}

pub type GameCell = Rc<RefCell<GameData>>;

pub fn create() -> GameCell {
    Rc::new(RefCell::new(GameData::new()))
}
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::IterMut;

use render::*;

/// Trait for each render command
pub trait GLCommand: 'static {
    fn process(&mut self, ll: &mut LowLevel);
}


/// Structure to store an manage GLCommand
pub struct GLCommandStore {
    commands: Vec<Box<GLCommand>>,
}

impl GLCommandStore {
    pub fn new() -> Rc<RefCell<GLCommandStore>> {
        Rc::new(RefCell::new(GLCommandStore {
            commands: vec!(),
        }))
    }

    pub fn add<Cmd: GLCommand>(&mut self, cmd: Cmd) {
        self.commands.push(Box::new(cmd));
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Box<GLCommand>> {
        self.commands.iter_mut()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}


/// Structure to wrap a GLCommandStore into a sharable CommandStore
pub type CommandStoreImpl = GLCommandStore;
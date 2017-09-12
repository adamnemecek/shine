use std::slice::IterMut;

use render::*;

/// Trait for each render command
pub trait Command: 'static {
    fn process(&mut self, ll: &mut LowLevel);
}


/// Structure to store an manage GLCommand
pub struct CommandStore {
    commands: Vec<Box<Command>>,
}

impl CommandStore {
    pub fn new() -> CommandStore {
        CommandStore {
            commands: vec!(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Box<Command>> {
        self.commands.iter_mut()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}


impl CommandQueue for CommandStore {
    fn add<C: Command>(&mut self, cmd: C) {
        self.commands.push(Box::new(cmd));
    }
}
extern crate gl;

use std::slice::IterMut;

use render::*;
use render::gl::*;


pub trait ICommand: 'static {
    fn process(&mut self, ll: &mut LowLevel);
}

pub struct CommandQueue {
    commands: Vec<Box<ICommand>>,
}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue {
            commands: vec!(),
        }
    }

    pub fn add<Cmd: ICommand>(&mut self, cmd: Cmd) {
        self.commands.push(Box::new(cmd));
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Box<ICommand>> {
        self.commands.iter_mut()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {}
}

impl ICommandQueue for CommandQueue {}
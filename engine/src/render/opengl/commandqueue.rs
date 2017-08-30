use std::slice::IterMut;

use render::*;

pub trait GLCommand: 'static {
    fn process(&mut self, ll: &mut LowLevel);
}

pub struct GLCommandQueue {
    commands: Vec<Box<GLCommand>>,
}

impl GLCommandQueue {
    pub fn new() -> GLCommandQueue {
        GLCommandQueue {
            commands: vec!(),
        }
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

pub type CommandQueueImpl = GLCommandQueue;

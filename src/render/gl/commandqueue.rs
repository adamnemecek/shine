#![allow(dead_code)]
extern crate gl;

use render::*;
use super::lowlevel::LowLevel;

pub trait ICommand: 'static {
    fn process(&mut self, ll: &mut LowLevel);
}

pub struct CommandQueue {
   pub commands: Vec<Box<ICommand>>,
}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue{
            commands: vec!(),
        }
    }

    pub fn add<Cmd: ICommand>(&mut self, cmd: Cmd) {
        self.commands.push(Box::new(cmd));
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {}
}

impl ICommandQueue for CommandQueue {
}
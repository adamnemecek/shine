use render::*;

pub trait CommandQueue {
    fn add<C: Command>(&mut self, cmd: C);
}

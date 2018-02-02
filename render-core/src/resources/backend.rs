pub trait Backend: 'static {
    ///Trait to abstract the command queue.
    type CommandQueue;

    ///Trait to abstract low-level platform state changes.
    type CommandContext;

    fn get_queue(&self) -> Self::CommandQueue;
    fn flush(&mut self);
}

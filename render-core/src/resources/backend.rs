pub trait Backend: 'static {
    type CommandQueue;

    fn get_queue(&self) -> Self::CommandQueue;
    fn flush(&mut self);
}

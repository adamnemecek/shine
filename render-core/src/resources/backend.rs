pub trait Backend: 'static {
    type FrameCompose;

    fn compose(&self) -> Self::FrameCompose;
    fn flush(&mut self);
}

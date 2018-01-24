pub trait FrameCompose {
    fn flush(&mut self);
}

pub trait Backend: 'static {
    type FrameCompose: FrameCompose;

    fn compose(&mut self) -> Self::FrameCompose;
}

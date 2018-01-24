pub trait FrameCompose {}

pub trait Backend: 'static {
    type FrameCompose: FrameCompose;

    fn compose(&mut self) -> Self::FrameCompose;
    fn present(&mut self);
}

pub struct FrameInfo {
    pub frame_id: u32,
    pub ellapsed_time: f64,
}

impl FrameInfo {
    pub fn new() -> FrameInfo {
        FrameInfo {
            frame_id: 1,
            ellapsed_time: 0.,
        }
    }
}

pub struct FrameInfo {
    pub frame_id: u32,
    pub elapsed_time: f32,
}

impl FrameInfo {
    pub fn new() -> FrameInfo {
        FrameInfo {
            frame_id: 1,
            elapsed_time: 0.,
        }
    }
}

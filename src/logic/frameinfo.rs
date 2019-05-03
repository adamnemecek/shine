use shine_stdext::time::FrameTimer;
use std::time::Duration;

/// Common configuration for the logic frames
pub struct FrameInfo {
    pub logic_frame_length: Duration,
    pub world_frame_length: Duration,
    pub frame_id: u32,
    pub frame_timer: FrameTimer,
}

impl FrameInfo {
    pub fn new() -> FrameInfo {
        FrameInfo {
            logic_frame_length: Duration::from_millis(100),
            world_frame_length: Duration::from_millis(100),
            frame_id: 0,
            frame_timer: FrameTimer::new(),
        }
    }

    pub fn start_frame(&mut self) {
        self.frame_id += 1;
        self.frame_timer.start();
    }
}

impl Default for FrameInfo {
    fn default() -> Self {
        FrameInfo::new()
    }
}

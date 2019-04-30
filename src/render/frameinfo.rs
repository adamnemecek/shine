use shine_stdext::time::{FrameTimer, LogicTimer};
use std::time::Duration;

#[derive(Debug)]
pub struct FrameInfo {
    pub logic_frame_id: u32,
    pub logic_timer: LogicTimer,
    pub frame_id: u32,
    pub frame_timer: FrameTimer,
}

impl FrameInfo {
    pub fn new() -> FrameInfo {
        FrameInfo {
            logic_frame_id: 0,
            logic_timer: LogicTimer::new(),
            frame_id: 0,
            frame_timer: FrameTimer::new(),
        }
    }

    pub fn start_logic(&mut self, world_frame_length: Duration) {
        self.logic_frame_id += 1;
        self.logic_timer.start_logic(world_frame_length);        
    }

    pub fn start_frame(&mut self) {
        self.frame_id += 1;
        self.frame_timer.start();
    }
}

use std::time::{Duration, Instant};
use std::fmt;

/// Measure the time of a single frame
#[derive(Debug)]
pub struct FrameTimer {
    frame_start: Option<Instant>,
    prev_frame_length: Duration,
}

impl FrameTimer {
    pub fn new() -> FrameTimer {
        FrameTimer {
            frame_start: None,
            prev_frame_length: Duration::default(),
        }
    }

    pub fn start(&mut self) {
        if let Some(prev_frame_start) = self.frame_start.replace(Instant::now()) {
            self.prev_frame_length = self.frame_start.map(|v| v.duration_since(prev_frame_start)).unwrap();
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.prev_frame_length
    }

    pub fn exact_elapsed(&self) -> Duration {
        self.frame_start.map(|v| v.elapsed()).unwrap_or(Duration::default())
    }
}

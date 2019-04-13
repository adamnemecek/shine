use std::time::{Duration, Instant};

pub struct FrameStatistics {
    prev_frame_instant: Option<Instant>,
    frame_count: u32,
    last_frame_time: Duration,
}

impl FrameStatistics {
    pub fn new() -> FrameStatistics {
        FrameStatistics {
            prev_frame_instant: None,
            frame_count: 0,
            last_frame_time: Duration::default(),
        }
    }

    pub fn get_last_frame_time(&self) -> Duration {
        self.last_frame_time
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn end_frame(&mut self) {
        self.last_frame_time = {
            match self.prev_frame_instant.replace(Instant::now()) {
                None => Duration::default(),
                Some(prev) => prev.elapsed(),
            }
        };

        if self.last_frame_time > Duration::from_millis(20) {
            log::trace!("too long frame: {:?}", self.last_frame_time);
        }

        self.frame_count += 1;
    }
}

impl Default for FrameStatistics {
    fn default() -> Self {
        FrameStatistics::new()
    }
}

use std::time::Instant;

pub struct FrameTimer {
    prev_frame_instant: Option<Instant>,
    frame_count: u32,
    last_frame_time: f64,
}

impl FrameTimer {
    pub fn new() -> FrameTimer {
        FrameTimer {
            prev_frame_instant: None,
            frame_count: 0,
            last_frame_time: 0.,
        }
    }

    pub fn get_last_frame_time(&self) -> f64 {
        self.last_frame_time
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn end_frame(&mut self) {
        self.last_frame_time = {
            match self.prev_frame_instant.replace(Instant::now()) {
                None => 0.0_f64,
                Some(prev) => prev.elapsed().as_micros() as f64,
            }
        };

        if self.last_frame_time > 10000. {
            log::trace!("too long frame: {}ms", self.last_frame_time / 1000.);
        }

        self.frame_count += 1;
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        FrameTimer::new()
    }
}

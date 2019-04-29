use crate::time::{BlendScale, ScaledBlender};
use std::time::{Duration, Instant};

/// Measure time for intra-frame interpolations
#[derive(Debug)]
pub struct LogicTimer {
    start: Instant,
    frame_length: Duration,
    prev_blend: f32,
    cur_blend: f32,
}

impl LogicTimer {
    pub fn new(frame_length: Duration) -> LogicTimer {
        LogicTimer {
            start: Instant::now(),
            frame_length,
            prev_blend: 0.,
            cur_blend: 0.,
        }
    }

    pub fn start_logic(&mut self, frame_length: Duration) {
        self.start = Instant::now();
        self.frame_length = frame_length;
        self.prev_blend = 0.;
        self.cur_blend = 0.;
    }

    pub fn start_frame(&mut self) {
        self.prev_blend = self.cur_blend;

        let duration = self.start.elapsed();
        self.cur_blend = if duration >= self.frame_length {
            1.
        } else {
            let blend = (duration.as_micros() as f32) / (self.frame_length.as_micros() as f32);
            if blend > self.prev_blend {
                blend
            } else {
                self.prev_blend
            }
        };
    }

    pub fn get_blender(&self) -> ScaledBlender<()> {
        ScaledBlender::new(self.prev_blend, self.cur_blend, ())
    }

    pub fn get_scaled_blender<F: BlendScale>(&self, scale: F) -> ScaledBlender<F> {
        ScaledBlender::new(self.prev_blend, self.cur_blend, scale)
    }
}

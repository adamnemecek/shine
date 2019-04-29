use std::time::{Duration, Instant};

/// Scale function: [0..1]->[0..1]
pub trait BlendScale {
    fn scale(&self, x: f32) -> f32;
}

impl BlendScale for Fn(f32) -> f32 {
    fn scale(&self, x: f32) -> f32 {
        (*self)(x)
    }
}

impl BlendScale for () {
    fn scale(&self, x: f32) -> f32 {
        x
    }
}

/// Helper to blend by elapsed time.
pub struct FrameBlender<S = ()>
where
    S: BlendScale,
{
    prev: f32,
    cur: f32,
    scale: S,
}

impl<S> FrameBlender<S>
where
    S: BlendScale,
{
    pub fn get_start_end_weight(&self) -> (f32, f32) {
        let w = self.scale.scale(self.cur);
        (1. - w, w)
    }

    pub fn get_prev_end_weight(&self) -> (f32, f32) {
        const LIMIT: f32 = 1. - 1e-6;

        let a = self.scale.scale(self.prev);
        assert!(a >= 0. && a <= 1.);
        let b = self.scale.scale(self.cur);
        assert!(b >= 0. && b <= 1.);
        assert!(a <= b);

        if b > LIMIT {
            (0., 1.)
        } else {
            let w0 = (1. - b) / (1. - a);
            let w1 = b - w0;
            (w0, w1)
        }
    }

    pub fn blend_start_end<T, F: FnMut(f32, f32) -> T>(&self, mut blend: F) -> T {
        let (w0, w1) = self.get_start_end_weight();
        blend(w0, w1)
    }

    pub fn blend_prev_end<T, F: FnMut(f32, f32) -> T>(&self, mut blend: F) -> T {
        let (w0, w1) = self.get_prev_end_weight();
        blend(w0, w1)
    }
}

/// Measure time for intra-frame interpolations
pub struct FrameTimer {
    start_instant: Instant,
    frame_length: Duration,
    prev_blend: f32,
    cur_blend: f32,
}

impl FrameTimer {
    pub fn new(frame_length: Duration) -> FrameTimer {
        FrameTimer {
            start_instant: Instant::now(),
            frame_length,
            prev_blend: 0.,
            cur_blend: 0.,
        }
    }

    pub fn start(&mut self, frame_length: Duration) {
        self.frame_length = frame_length;
        self.prev_blend = 0.;
        self.cur_blend = 0.;
        self.start_instant = Instant::now();
    }

    pub fn advance(&mut self) {
        self.prev_blend = self.cur_blend;

        let duration = self.start_instant.elapsed();
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

    pub fn get_blender(&self) -> FrameBlender<()> {
        FrameBlender {
            prev: self.prev_blend,
            cur: self.cur_blend,
            scale: (),
        }
    }

    pub fn get_scaled_blender<F: BlendScale>(&self, scale: F) -> FrameBlender<F> {
        FrameBlender {
            prev: self.prev_blend,
            cur: self.cur_blend,
            scale,
        }
    }
}

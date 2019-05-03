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
pub struct ScaledBlender<S = ()>
where
    S: BlendScale,
{
    prev: f32,
    cur: f32,
    scale: S,
}

impl<S> ScaledBlender<S>
where
    S: BlendScale,
{
    pub fn new(prev: f32, cur: f32, scale: S) -> ScaledBlender<S> {
        assert!(prev >= 0. && prev <= 1.);
        assert!(cur >= 0. && cur <= 1.);
        assert!(prev <= cur);

        ScaledBlender { prev, cur, scale }
    }

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

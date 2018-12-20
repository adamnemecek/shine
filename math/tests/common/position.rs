use shine_math::geometry::{Posf32, Posf64, Posi32, Posi64};

/// Sample points for test cases
pub struct Sample(pub f32, pub f32);

impl From<Sample> for Posf32 {
    fn from(p: Sample) -> Posf32 {
        Posf32 {
            x: p.0 as f32,
            y: p.1 as f32,
        }
    }
}

impl From<Sample> for Posf64 {
    fn from(p: Sample) -> Posf64 {
        Posf64 {
            x: p.0 as f64,
            y: p.1 as f64,
        }
    }
}

impl From<Sample> for Posi32 {
    fn from(p: Sample) -> Posi32 {
        Posi32 {
            x: (p.0 * 2048.) as i32,
            y: (p.1 * 2048.) as i32,
        }
    }
}

impl From<Sample> for Posi64 {
    fn from(p: Sample) -> Posi64 {
        Posi64 {
            x: (p.0 * 65536.) as i64,
            y: (p.1 * 65536.) as i64,
        }
    }
}

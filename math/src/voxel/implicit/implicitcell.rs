use crate::voxel::implicit::Function;
use crate::voxel::Cell;
use std::ops::Range;

/// Generate function from implicit function.
pub struct ImplicitCell<F>
where
    F: Function,
{
    /// The range of the cell that is mapped into the [-1,1] cube before function evaluation.
    range: (Range<isize>, Range<isize>, Range<isize>),

    /// Helper to map from the cell range into the [-1,1] cube
    range_map: ((f32, f32), (f32, f32), (f32, f32)),

    /// The function to evaluate
    function: F,

    /// Invert the inside/outside relation
    invert: bool,

    /// The clamp value for voxel border
    clamp: f32,
}

impl<F> ImplicitCell<F>
where
    F: Function,
{
    pub fn new(function: F) -> ImplicitCell<F> {
        ImplicitCell {
            range: (0isize..32isize, 0isize..32isize, 0isize..32isize),
            range_map: ((0., 1. / 31.), (0., 1. / 31.), (0., 1. / 31.)),
            function,
            clamp: 0.,
            invert: false,
        }
    }

    pub fn with_x_range(self, start: isize, end: isize) -> Self {
        assert!(end > start);
        ImplicitCell {
            range: (start..end, self.range.1, self.range.2),
            range_map: (((1 - start) as f32, (end - start) as f32), self.range_map.1, self.range_map.2),
            ..self
        }
    }

    pub fn with_y_range(self, start: isize, end: isize) -> Self {
        ImplicitCell {
            range: (self.range.0, start..end, self.range.2),
            range_map: (self.range_map.0, ((1 - start) as f32, (end - start) as f32), self.range_map.2),
            ..self
        }
    }

    pub fn with_z_range(self, start: isize, end: isize) -> Self {
        ImplicitCell {
            range: (self.range.0, self.range.1, start..end),
            range_map: (self.range_map.0, self.range_map.1, ((1 - start) as f32, (end - start) as f32)),
            ..self
        }
    }

    pub fn with_clamp(self, clamp: f32) -> Self {
        ImplicitCell { clamp, ..self }
    }

    pub fn with_invert(self) -> Self {
        ImplicitCell { invert: true, ..self }
    }
}

impl<F> Cell for ImplicitCell<F>
where
    F: Function,
{
    fn x_range(&self) -> Range<isize> {
        self.range.0.clone()
    }

    fn y_range(&self) -> Range<isize> {
        self.range.1.clone()
    }

    fn z_range(&self) -> Range<isize> {
        self.range.2.clone()
    }

    fn get(&self, _lod: u32, x: isize, y: isize, z: isize) -> bool {
        let x = ((x as f32) + (self.range_map.0).0) * (self.range_map.0).1;
        let y = ((y as f32) + (self.range_map.1).0) * (self.range_map.1).1;
        let z = ((z as f32) + (self.range_map.2).0) * (self.range_map.2).1;
        let x = x * 2. - 1.;
        let y = y * 2. - 1.;
        let z = z * 2. - 1.;
        self.invert == (self.function.eval(x, y, z) > self.clamp)
    }
}

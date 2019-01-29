use crate::voxel::Cell;

pub trait Info {
    fn extremals(&self) -> (i16, i16);
}

impl<T> Info for T
where
    T: Cell,
{
    fn extremals(&self) -> (i16, i16) {
        let (sx, sy, sz) = self.resolution();

        let mut min = self.get(0, 0, 0, 0);
        let mut max = self.get(0, 0, 0, 0);
        for z in 0isize..(sz as isize) {
            for y in 0isize..(sy as isize) {
                for x in 0isize..(sx as isize) {
                    let v = self.get(0, x, y, z);
                    if v > max {
                        max = v;
                    };
                    if v < min {
                        min = v;
                    };
                }
            }
        }

        (min, max)
    }
}

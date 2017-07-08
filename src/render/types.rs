#[repr(C)]
#[derive(Debug)]
pub struct Float32x4(f32, f32, f32, f32);

impl Float32x4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Float32x4 {
        Float32x4(x, y, z, w)
    }
}

impl Default for Float32x4 {
    fn default() -> Float32x4 {
        Float32x4(0., 0., 0., 0.)
    }
}

#[macro_export]
macro_rules! f32x4 {
    ($_x:expr, $_y:expr, $_z:expr, $_w:expr) => { Float32x4::new($_x as f32, $_y as f32, $_z as f32, $_w as f32) };
    ($_x:expr) => { Float32x4::new($_x as f32, $_x as f32, $_x as f32, $_x as f32) };
    () => { Default::default() };
}


#[repr(C)]
#[derive(Debug)]
pub struct Float32x3(f32, f32, f32);

impl Float32x3 {
    pub fn new(x: f32, y: f32, z: f32) -> Float32x3 {
        Float32x3(x, y, z)
    }
}

impl Default for Float32x3 {
    fn default() -> Float32x3 {
        Float32x3(0., 0., 0.)
    }
}

#[macro_export]
macro_rules! f32x3 {
    ($_x:expr, $_y:expr, $_z:expr) => { Float32x3::new($_x as f32, $_y as f32, $_z as f32) };
    ($_x:expr) => { Float32x3::new($_x as f32, $_x as f32, $_x as f32) };
    () => { Default::default() };
}


#[repr(C)]
#[derive(Debug)]
pub struct Float32x2(f32, f32);

impl Float32x2 {
    pub fn new(x: f32, y: f32) -> Float32x2 {
        Float32x2(x, y)
    }
}

impl Default for Float32x2 {
    fn default() -> Float32x2 {
        Float32x2(0., 0.)
    }
}

#[macro_export]
macro_rules! f32x2 {
    ($_x:expr, $_y:expr) => { Float32x2::new($_x as f32, $_y as f32) };
    ($_x:expr) => { Float32x2::new($_x as f32, $_x as f32) };
    () => { Default::default() };
}


#[repr(C)]
#[derive(Debug)]
pub struct Float32x4(f32,f32,f32,f32);

impl Float32x4 {
    /*pub fn to_bytes(&self) -> &[u8] {
        let p: *const Self = self;
        let p = p as *const u8;
        unsafe{
            slice::from_raw_parts(p, mem::size_of::<Self>())
        }
    }*/
}

impl Default for Float32x4 {
    fn default() -> Float32x4 {
        Float32x4(0.,0.,0.,0.)
    }
}

macro_rules! f32x4 {
    ($_x:expr, $_y:expr, $_z:expr, $_w:expr) => { Float32x4($_x as f32, $_y as f32, $_z as f32, $_w as f32) };
    ($_x:expr) => { Float32x4($_x as f32, $_x as f32, $_x as f32, $_x as f32) };
    () => { Default::default() };
}

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]


/// Structure to store a window size.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    /// The width.
    pub width: i32,
    /// The height.
    pub height: i32,
}

impl From<[i32; 2]> for Size {
    #[inline(always)]
    fn from(value: [i32; 2]) -> Size {
        Size {
            width: value[0],
            height: value[1],
        }
    }
}

impl From<(i32, i32)> for Size {
    #[inline(always)]
    fn from(value: (i32, i32)) -> Size {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<Size> for [i32; 2] {
    #[inline(always)]
    fn from(value: Size) -> [i32; 2] {
        [value.width, value.height]
    }
}

impl From<Size> for (i32, i32) {
    #[inline(always)]
    fn from(value: Size) -> (i32, i32) {
        (value.width, value.height)
    }
}


/// Structure to store the window position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

impl From<[i32; 2]> for Position {
    #[inline(always)]
    fn from(value: [i32; 2]) -> Position {
        Position {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<(i32, i32)> for Position {
    #[inline(always)]
    fn from(value: (i32, i32)) -> Position {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Position> for [i32; 2] {
    #[inline(always)]
    fn from(value: Position) -> [i32; 2] {
        [value.x, value.y]
    }
}

impl From<Position> for (i32, i32) {
    #[inline(always)]
    fn from(value: Position) -> (i32, i32) {
        (value.x, value.y)
    }
}


/// Structure to store the window rectangle.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    /// The position of the top left corner.
    pub position: Position,
    /// The size of the rectangle
    pub size: Size,
}

impl From<[i32; 4]> for Rectangle {
    #[inline(always)]
    fn from(value: [i32; 4]) -> Rectangle {
        Rectangle {
            position: Position {
                x: value[0],
                y: value[1],
            },

            size: Size {
                width: value[2],
                height: value[3]
            }
        }
    }
}

impl From<(Position, Size)> for Rectangle {
    #[inline(always)]
    fn from(value: (Position, Size)) -> Rectangle {
        Rectangle {
            position: value.0,
            size: value.1,
        }
    }
}

impl From<Rectangle> for [i32; 4] {
    #[inline(always)]
    fn from(value: Rectangle) -> [i32; 4] {
        [value.position.x, value.position.y, value.size.width, value.size.height]
    }
}

impl From<Rectangle> for (Position, Size) {
    #[inline(always)]
    fn from(value: Rectangle) -> (Position, Size) {
        (value.position, value.size)
    }
}


/// Enum to define the semantics of raw image data.
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat {
    /// One byte components for Red,Green,Blue image.
    Rgb8,
    /// One byte components for Red,Green,Blue,Alpha image.
    Rgba8,
}


/// Geometry primitive type
#[derive(Copy, Clone, Debug)]
pub enum Primitive {
    /// Point primitive (1 vertex per primitive)
    Point,
    /// Line primitive (2 vertex per primitive)
    Line,
    /// Triangle primitive (3 vertex per primitive)
    Triangle,
}


/// Float array with 16 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float32x16(pub f32, pub f32, pub f32, pub f32,
                      pub f32, pub f32, pub f32, pub f32,
                      pub f32, pub f32, pub f32, pub f32,
                      pub f32, pub f32, pub f32, pub f32);

impl Default for Float32x16 {
    #[inline(always)]
    fn default() -> Float32x16 {
        Float32x16(0., 0., 0., 0.,
                   0., 0., 0., 0.,
                   0., 0., 0., 0.,
                   0., 0., 0., 0.)
    }
}

impl From<[f32; 16]> for Float32x16 {
    #[inline(always)]
    fn from(value: [f32; 16]) -> Float32x16 {
        Float32x16(value[0], value[1], value[2], value[3],
                   value[4], value[5], value[6], value[7],
                   value[8], value[9], value[10], value[11],
                   value[12], value[13], value[14], value[15])
    }
}

impl From<(f32, f32, f32, f32,
           f32, f32, f32, f32,
           f32, f32, f32, f32,
           f32, f32, f32, f32)> for Float32x16 {
    #[inline(always)]
    fn from(value: (f32, f32, f32, f32,
                    f32, f32, f32, f32,
                    f32, f32, f32, f32,
                    f32, f32, f32, f32)) -> Float32x16 {
        Float32x16(value.0, value.1, value.2, value.3,
                   value.4, value.5, value.6, value.7,
                   value.8, value.9, value.10, value.11,
                   value.12, value.13, value.14, value.15)
    }
}

impl From<(Float32x4,
           Float32x4,
           Float32x4,
           Float32x4)> for Float32x16 {
    #[inline(always)]
    fn from(value: (Float32x4, Float32x4, Float32x4, Float32x4)) -> Float32x16 {
        Float32x16((value.0).0, (value.0).1, (value.0).2, (value.0).3,
                   (value.1).0, (value.1).1, (value.1).2, (value.1).3,
                   (value.2).0, (value.2).1, (value.2).2, (value.2).3,
                   (value.3).0, (value.3).1, (value.3).2, (value.3).3)
    }
}

#[macro_export]
macro_rules! f32x16 {
    ($_a0:expr,  $_a1:expr,  $_a2:expr,  $_a3:expr,
     $_a4:expr,  $_a5:expr,  $_a6:expr,  $_a7:expr,
     $_a8:expr,  $_a9:expr,  $_a10:expr, $_a11:expr,
     $_a12:expr, $_a13:expr, $_a14:expr, $_a15:expr) => {
      $crate::Float32x16(
        $_a0 as f32,$_a1 as f32,$_a2 as f32,$_a3 as f32,
        $_a4 as f32,$_a5 as f32,$_a6 as f32,$_a7 as f32,
        $_a8 as f32,$_a9 as f32,$_a10 as f32,$_a11 as f32,
        $_a12 as f32,$_a13 as f32,$_a14 as f32,$_a15 as f32)
      };

    ($_x:expr) => {
      $crate::Float32x16(
        $_x as f32, $_x as f32, $_x as f32, $_x as f32,
        $_x as f32, $_x as f32, $_x as f32, $_x as f32,
        $_x as f32, $_x as f32, $_x as f32, $_x as f32,
        $_x as f32, $_x as f32, $_x as f32, $_x as f32)
      };

    () => { {let f : $crate::Float32x16 = Default::default(); f} };
}


/// Float array with 4 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float32x4(pub f32, pub f32, pub f32, pub f32);

impl Default for Float32x4 {
    #[inline(always)]
    fn default() -> Float32x4 {
        Float32x4(0., 0., 0., 0.)
    }
}

impl From<[f32; 4]> for Float32x4 {
    #[inline(always)]
    fn from(value: [f32; 4]) -> Float32x4 {
        Float32x4(value[0], value[1], value[2], value[3])
    }
}

impl From<(f32, f32, f32, f32)> for Float32x4 {
    #[inline(always)]
    fn from(value: (f32, f32, f32, f32)) -> Float32x4 {
        Float32x4(value.0, value.1, value.2, value.3)
    }
}

#[macro_export]
macro_rules! f32x4 {
    ($_x:expr, $_y:expr, $_z:expr, $_w:expr) => { $crate::Float32x4($_x as f32, $_y as f32, $_z as f32, $_w as f32) };
    ($_x:expr) => { $crate::Float32x4($_x as f32, $_x as f32, $_x as f32, $_x as f32) };
    () => { {let f : $crate::Float32x4 = Default::default(); f} };
}


/// Float array with 3 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float32x3(pub f32, pub f32, pub f32);

impl Default for Float32x3 {
    fn default() -> Float32x3 {
        Float32x3(0., 0., 0.)
    }
}

impl From<[f32; 3]> for Float32x3 {
    #[inline(always)]
    fn from(value: [f32; 3]) -> Float32x3 {
        Float32x3(value[0], value[1], value[2])
    }
}

impl From<(f32, f32, f32)> for Float32x3 {
    #[inline(always)]
    fn from(value: (f32, f32, f32)) -> Float32x3 {
        Float32x3(value.0, value.1, value.2)
    }
}

#[macro_export]
macro_rules! f32x3 {
    ($_x:expr, $_y:expr, $_z:expr) => { $crate::Float32x3($_x as f32, $_y as f32, $_z as f32) };
    ($_x:expr) => { $crate::Float32x3::new($_x as f32, $_x as f32, $_x as f32) };
    () => { {let f : $crate::Float32x3 = Default::default(); f} };
}


/// Float array with 2 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Float32x2(pub f32, pub f32);

impl Default for Float32x2 {
    fn default() -> Float32x2 {
        Float32x2(0., 0.)
    }
}

impl From<[f32; 2]> for Float32x2 {
    #[inline(always)]
    fn from(value: [f32; 2]) -> Float32x2 {
        Float32x2(value[0], value[1])
    }
}

impl From<(f32, f32)> for Float32x2 {
    #[inline(always)]
    fn from(value: (f32, f32)) -> Float32x2 {
        Float32x2(value.0, value.1)
    }
}

#[macro_export]
macro_rules! f32x2 {
    ($_x:expr, $_y:expr) => { $crate::Float32x2($_x as f32, $_y as f32) };
    ($_x:expr) => { $crate::Float32x2::new($_x as f32, $_x as f32) };
    () => { {let f : $crate::Float32x2 = Default::default(); f} };
}

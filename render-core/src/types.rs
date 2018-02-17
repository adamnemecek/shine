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
                height: value[3],
            },
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
    /// 8bit grayscale image.
    R8,
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


/// Unsigned u8 array with 4 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UInt8x4(pub u8, pub u8, pub u8, pub u8);

impl Default for UInt8x4 {
    fn default() -> UInt8x4 {
        UInt8x4(0, 0, 0, 0)
    }
}

impl From<[u8; 4]> for UInt8x4 {
    #[inline(always)]
    fn from(value: [u8; 4]) -> UInt8x4 {
        UInt8x4(value[0], value[1], value[2], value[3])
    }
}

impl From<(u8, u8, u8, u8)> for UInt8x4 {
    #[inline(always)]
    fn from(value: (u8, u8, u8, u8)) -> UInt8x4 {
        UInt8x4(value.0, value.1, value.2, value.3)
    }
}

impl From<u32> for UInt8x4 {
    #[inline(always)]
    fn from(value: u32) -> UInt8x4 {
        UInt8x4(((value >> 24) & 0xff) as u8, ((value >> 16) & 0xff) as u8, ((value >> 8) & 0xff) as u8, (value & 0xff) as u8)
    }
}


/// Unsigned u8 array with 3 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UInt8x3(pub u8, pub u8, pub u8);

impl Default for UInt8x3 {
    fn default() -> UInt8x3 {
        UInt8x3(0, 0, 0)
    }
}

impl From<[u8; 3]> for UInt8x3 {
    #[inline(always)]
    fn from(value: [u8; 3]) -> UInt8x3 {
        UInt8x3(value[0], value[1], value[2])
    }
}

impl From<(u8, u8, u8)> for UInt8x3 {
    #[inline(always)]
    fn from(value: (u8, u8, u8)) -> UInt8x3 {
        UInt8x3(value.0, value.1, value.2)
    }
}


/// Unsigned u8 array with 2 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UInt8x2(pub u8, pub u8);

impl Default for UInt8x2 {
    fn default() -> UInt8x2 {
        UInt8x2(0, 0)
    }
}

impl From<[u8; 2]> for UInt8x2 {
    #[inline(always)]
    fn from(value: [u8; 2]) -> UInt8x2 {
        UInt8x2(value[0], value[1])
    }
}

impl From<(u8, u8)> for UInt8x2 {
    #[inline(always)]
    fn from(value: (u8, u8)) -> UInt8x2 {
        UInt8x2(value.0, value.1)
    }
}












/// Unsigned normalized (fixed point array) u8 array with 4 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct NUInt8x4(pub u8, pub u8, pub u8, pub u8);

impl Default for NUInt8x4 {
    fn default() -> NUInt8x4 {
        NUInt8x4(0, 0, 0, 0)
    }
}

impl From<[u8; 4]> for NUInt8x4 {
    #[inline(always)]
    fn from(value: [u8; 4]) -> NUInt8x4 {
        NUInt8x4(value[0], value[1], value[2], value[3])
    }
}

impl From<(u8, u8, u8, u8)> for NUInt8x4 {
    #[inline(always)]
    fn from(value: (u8, u8, u8, u8)) -> NUInt8x4 {
        NUInt8x4(value.0, value.1, value.2, value.3)
    }
}

impl From<u32> for NUInt8x4 {
    #[inline(always)]
    fn from(value: u32) -> NUInt8x4 {
        NUInt8x4(((value >> 24) & 0xff) as u8, ((value >> 16) & 0xff) as u8, ((value >> 8) & 0xff) as u8, (value & 0xff) as u8)
    }
}


/// Unsigned normalized (fixed point array) u8 array with 3 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct NUInt8x3(pub u8, pub u8, pub u8);

impl Default for NUInt8x3 {
    fn default() -> NUInt8x3 {
        NUInt8x3(0, 0, 0)
    }
}

impl From<[u8; 3]> for NUInt8x3 {
    #[inline(always)]
    fn from(value: [u8; 3]) -> NUInt8x3 {
        NUInt8x3(value[0], value[1], value[2])
    }
}

impl From<(u8, u8, u8)> for NUInt8x3 {
    #[inline(always)]
    fn from(value: (u8, u8, u8)) -> NUInt8x3 {
        NUInt8x3(value.0, value.1, value.2)
    }
}


/// Unsigned normalized (fixed point array) u8 array with 2 components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct NUInt8x2(pub u8, pub u8);

impl Default for NUInt8x2 {
    fn default() -> NUInt8x2 {
        NUInt8x2(0, 0)
    }
}

impl From<[u8; 2]> for NUInt8x2 {
    #[inline(always)]
    fn from(value: [u8; 2]) -> NUInt8x2 {
        NUInt8x2(value[0], value[1])
    }
}

impl From<(u8, u8)> for NUInt8x2 {
    #[inline(always)]
    fn from(value: (u8, u8)) -> NUInt8x2 {
        NUInt8x2(value.0, value.1)
    }
}

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::fmt;

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


/// Enum like trait that can iterate the values and can be convert to and from primitive type.
///
pub trait PrimitiveEnum: 'static + Copy + Clone + fmt::Debug {
    /// Creates a plain enum for its index
    ///
    /// If index is not in the range, None is returned
    fn from_index(index: usize) -> Option<Self> where Self: Sized {
        if index < Self::count() {
            Some(Self::from_index_unsafe(index))
        } else {
            None
        }
    }

    /// Creates a plain enum for its index
    ///
    /// It is unsafe in the sence that, no check is mde on the ranges.
    /// It is assumed a valid index is provided. If unsure, use the safe version.
    fn from_index_unsafe(index: usize) -> Self where Self: Sized;

    /// Creates enum value from its name
    fn from_name(name: &str) -> Option<Self> where Self: Sized;

    /// Converts an enum into a plain index
    fn to_index(&self) -> usize;

    /// Returns the number of attributes.
    fn count() -> usize;
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

/// Constructs Float32x4 from multiple expressions
/// # Example
/// assert!( f32x4!(1, 2, 3, 4) == Float32x4(1., 2., 3., 4.) );
/// assert!( f32x4!(1) == Float32x4(1., 1., 1, 1.) );
/// assert!( f32x4!() == Float32x4(0., 0., 0., 0.) );
#[macro_export]
macro_rules! f32x4 {
    ($_x:expr, $_y:expr, $_z:expr, $_w:expr) => { $crate::render::Float32x4($_x as f32, $_y as f32, $_z as f32, $_w as f32) };
    ($_x:expr) => { $crate::render::Float32x4($_x as f32, $_x as f32, $_x as f32, $_x as f32) };
    () => { {let f : $crate::render::Float32x4 = Default::default(); f} };
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

/// Constructs Float32x3 from multiple expressions
///
/// # Example
/// assert!( f32x3!(1, 2, 3) == Float32x3(1., 2., 3.) );
/// assert!( f32x3!(1) == Float32x3(1., 1., 1) );
/// assert!( f32x3!() == Float32x3(0., 0., 0.) );
#[macro_export]
macro_rules! f32x3 {
    ($_x:expr, $_y:expr, $_z:expr) => { $crate::render::Float32x3($_x as f32, $_y as f32, $_z as f32) };
    ($_x:expr) => { $crate::render::Float32x3::new($_x as f32, $_x as f32, $_x as f32) };
    () => { {let f : $crate::render::Float32x3 = Default::default(); f} };
}


/// Float array with 2 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Constructs Float32x3 from multiple expressions
///
/// # Example
/// assert!( f32x2!(1, 2) == Float32x2(1., 2.) );
/// assert!( f32x2!(1) == Float32x2(1., 1.) );
/// assert!( f32x2!() == Float32x2(0., 0.) );
#[macro_export]
macro_rules! f32x2 {
    ($_x:expr, $_y:expr) => { $crate::render::Float32x2($_x as f32, $_y as f32) };
    ($_x:expr) => { $crate::render::Float32x2::new($_x as f32, $_x as f32) };
    () => { {let f : $crate::render::Float32x2 = Default::default(); f} };
}


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

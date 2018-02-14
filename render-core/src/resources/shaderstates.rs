/// Viewport
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Viewport {
    FullScreen,
    /// Bottom, Left, Width, Height. Width Height must be positive
    Fixed(i32, i32, i32, i32),
    /// Bottom, Left, Width, Height in percent. Width Height must be positive
    Proportional(f32, f32, f32, f32),
}

impl Default for Viewport {
    fn default() -> Viewport {
        Viewport::FullScreen
    }
}


/// Depth function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepthFunction {
    Disable,
    Always,
    Never,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
}

impl Default for DepthFunction {
    fn default() -> DepthFunction {
        DepthFunction::Disable
    }
}


/// Depth function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CullFunction {
    Disable,
    Clockwise,
    CounterClockwise,
}

impl Default for CullFunction {
    fn default() -> CullFunction {
        CullFunction::Disable
    }
}


/// Blend function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlendFunction {
    Disable,
}

impl Default for BlendFunction {
    fn default() -> BlendFunction {
        BlendFunction::Disable
    }
}


/// Stencil function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StencilFunction {
    Disable,
    Always { ref_value: i32, mask_value: u32 },
    Never { ref_value: i32, mask_value: u32 },
    Less { ref_value: i32, mask_value: u32 },
    LessEqual { ref_value: i32, mask_value: u32 },
    Greater { ref_value: i32, mask_value: u32 },
    GreaterEqual { ref_value: i32, mask_value: u32 },
    Equal { ref_value: i32, mask_value: u32 },
    NotEqual { ref_value: i32, mask_value: u32 },
}

impl Default for StencilFunction {
    fn default() -> StencilFunction {
        StencilFunction::Disable
    }
}

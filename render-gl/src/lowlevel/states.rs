#![allow(dead_code, unused_variables)]

use lowlevel::*;

/// Viewport
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Viewport {
    FullScreen,
    //Propotional(f32, f32, f32, f32),
    //Fixed(i32, i32, i32, i32),
}

pub struct ViewportState {
    time_stamp: usize,
    function: Viewport,
    current: (i32, i32, i32, i32),
}

impl ViewportState {
    fn new() -> ViewportState {
        ViewportState {
            time_stamp: 0,
            function: Viewport::FullScreen,
            current: (0, 0, 0, 0),
        }
    }

    fn commit(&mut self, time: usize, screen_size: Size) {
        if self.time_stamp != time {
            self.function = Viewport::FullScreen;
        }
    }

    fn set(&mut self, time: usize, fun: Viewport) {
        self.function = fun;
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

pub struct DepthState {
    time_stamp: usize,
    function: DepthFunction,
}

impl DepthState {
    fn new() -> DepthState {
        DepthState {
            time_stamp: 0,
            function: DepthFunction::Disable,
        }
    }

    fn commit(&mut self, time: usize) {
        if self.time_stamp != time {
            self.set(time, DepthFunction::Less);
        }
    }

    fn set(&mut self, time: usize, fun: DepthFunction) {
        assert!(self.time_stamp != time, "state already set in this frame");

        self.time_stamp = time;
        if self.function == fun && time != 0 {
            return;
        }

        gl_check_error();
        match self.function {
            DepthFunction::Disable => {
                ugl!(Disable(gl::DEPTH_TEST));
            }
            DepthFunction::Always => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::ALWAYS));
            }
            DepthFunction::Never => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::NEVER));
            }
            DepthFunction::Less => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::LESS));
            }
            DepthFunction::LessEqual => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::LEQUAL));
            }
            DepthFunction::Greater => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::GREATER));
            }
            DepthFunction::GreaterEqual => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::GEQUAL));
            }
            DepthFunction::Equal => {
                ugl!(Enable(gl::DEPTH_TEST));
                ugl!(DepthFunc(gl::EQUAL));
            }
        }
        gl_check_error();
    }
}


/// Blend function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlendFunction {
    Disable,

}

pub struct BlendState {
    time_stamp: usize,
    function: BlendFunction,
}

impl BlendState {
    fn new() -> BlendState {
        BlendState {
            time_stamp: 0,
            function: BlendFunction::Disable,
        }
    }

    fn commit(&mut self, time: usize) {
        if self.time_stamp != time {
            self.set(time, BlendFunction::Disable);
        }
    }

    fn set(&mut self, time: usize, fun: BlendFunction) {
        assert!(self.time_stamp != time, "state already set in this frame");

        self.time_stamp = time;
        if self.function == fun && time != 0 {
            return;
        }

        gl_check_error();
        match self.function {
            BlendFunction::Disable => {
                ugl!(Disable(gl::BLEND));
            }
        }
        gl_check_error();
    }
}


/// Stencil function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StencilFunction {
    Disable,
    Always(GLint, GLuint),
    Never(GLint, GLuint),
    Less(GLint, GLuint),
    LessEqual(GLint, GLuint),
    Greater(GLint, GLuint),
    GreaterEqual(GLint, GLuint),
    Equal(GLint, GLuint),
    NotEqual(GLint, GLuint)
}

pub struct StencilState {
    time_stamp: usize,
    function: StencilFunction,
}

impl StencilState {
    fn new() -> StencilState {
        StencilState {
            time_stamp: 0,
            function: StencilFunction::Disable,
        }
    }

    fn commit(&mut self, time: usize) {
        if self.time_stamp != time {
            self.set(time, StencilFunction::Disable);
        }
    }

    fn set(&mut self, time: usize, fun: StencilFunction) {
        assert!(self.time_stamp != time, "state already set in this frame");

        self.time_stamp = time;
        if self.function == fun && time != 0 {
            return;
        }

        gl_check_error();
        match self.function {
            StencilFunction::Disable => {
                ugl!(Disable(gl::STENCIL_TEST));
            }
            StencilFunction::Always(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::ALWAYS, ref_value, mask_value));
            }
            StencilFunction::Never(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::NEVER, ref_value, mask_value));
            }
            StencilFunction::Less(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::LESS, ref_value, mask_value));
            }
            StencilFunction::LessEqual(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::LEQUAL, ref_value, mask_value));
            }
            StencilFunction::Greater(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::GREATER, ref_value, mask_value));
            }
            StencilFunction::GreaterEqual(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::GEQUAL, ref_value, mask_value));
            }
            StencilFunction::Equal(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::EQUAL, ref_value, mask_value));
            }
            StencilFunction::NotEqual(ref_value, mask_value) => {
                ugl!(Enable(gl::STENCIL_TEST));
                ugl!(StencilFunc(gl::NOTEQUAL, ref_value, mask_value));
            }
        }
        gl_check_error();
    }
}


/// Structure to handle gl states
pub struct StateManager {
    forced: bool,
    time_stamp: usize,
    pub depth: DepthState,
    pub stencil: StencilState,
    pub blend: BlendState,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            forced: false,
            time_stamp: 1,
            depth: DepthState::new(),
            stencil: StencilState::new(),
            blend: BlendState::new(),
        }
    }

    pub fn set_forced(&mut self, forced: bool) {
        self.forced = forced;
    }

    pub fn commit(&mut self) {
        self.depth.commit(self.time_stamp);
        self.stencil.commit(self.time_stamp);
        self.blend.commit(self.time_stamp);

        self.time_stamp = self.time_stamp.wrapping_add(1);
    }
}

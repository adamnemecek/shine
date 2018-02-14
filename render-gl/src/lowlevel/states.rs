#![allow(dead_code, unused_variables)]

use core::*;
use lowlevel::*;


/// Viewport state
#[derive(Debug)]
struct ViewportState {
    render_size: Size,
    function: Viewport,
    current: (i32, i32, i32, i32),
}

impl ViewportState {
    fn new() -> ViewportState {
        ViewportState {
            render_size: Size::from((0, 0)),
            function: Default::default(),
            current: (0, 0, 0, 0),
        }
    }

    fn commit(&mut self) {
        let screen = self.render_size;
        let vp =
            match self.function {
                Viewport::FullScreen => (0, 0, screen.width, screen.height),
                Viewport::Fixed(a, b, c, d) => (a, b, c, d),
                Viewport::Proportional(a, b, c, d) => ((screen.width as f32 * a + 0.5) as i32,
                                                       (screen.height as f32 * b + 0.5) as i32,
                                                       (screen.width as f32 * c + 0.5) as i32,
                                                       (screen.height as f32 * d + 0.5) as i32),
            };

        if vp != self.current {
            self.current = vp;
            assert!(vp.2 > 0 || vp.3 > 0, "non-positive width or height parameter for the viewport: {:?}", vp);
            gl_check_error();
            ffi!(gl::Viewport(vp.0, vp.1, vp.2, vp.3));
            gl_check_error();
        }
    }
}


/// Depth function state
#[derive(Debug)]
struct DepthState {
    value: DepthFunction,
}

impl DepthState {
    fn commit(&self) {
        gl_check_error();
        match self.value {
            DepthFunction::Disable => {
                ffi!(gl::Disable(gl::DEPTH_TEST));
            }
            DepthFunction::Always => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::ALWAYS));
            }
            DepthFunction::Never => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::NEVER));
            }
            DepthFunction::Less => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::LESS));
            }
            DepthFunction::LessEqual => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::LEQUAL));
            }
            DepthFunction::Greater => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::GREATER));
            }
            DepthFunction::GreaterEqual => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::GEQUAL));
            }
            DepthFunction::Equal => {
                ffi!(gl::Enable(gl::DEPTH_TEST));
                ffi!(gl::DepthFunc(gl::EQUAL));
            }
        }
        gl_check_error();
    }
}


/// Cull function state
#[derive(Debug)]
struct CullState {
    value: CullFunction,
}

impl CullState {
    fn commit(&self) {
        gl_check_error();
        match self.value {
            CullFunction::Disable => {
                ffi!(gl::Disable(gl::CULL_FACE));
            }
            CullFunction::Clockwise => {
                ffi!(gl::Enable(gl::CULL_FACE));
                ffi!(gl::CullFace(gl::BACK));
            }
            CullFunction::CounterClockwise => {
                ffi!(gl::Enable(gl::CULL_FACE));
                ffi!(gl::CullFace(gl::FRONT));
            }
        }
        gl_check_error();
    }
}


/// Blend function
#[derive(Debug)]
struct BlendState {
    value: BlendFunction,
}

impl BlendState {
    fn commit(&self) {
        gl_check_error();
        match self.value {
            BlendFunction::Disable => {
                ffi!(gl::Disable(gl::BLEND));
            }
        }
        gl_check_error();
    }
}


/// Stencil function
#[derive(Debug)]
pub struct StencilState {
    value: StencilFunction,
}

impl StencilState {
    fn commit(&self) {
        gl_check_error();
        match self.value {
            StencilFunction::Disable => {
                ffi!(gl::Disable(gl::STENCIL_TEST));
            }
            StencilFunction::Always { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::ALWAYS, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::Never { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::NEVER, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::Less { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::LESS, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::LessEqual { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::LEQUAL, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::Greater { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::GREATER, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::GreaterEqual { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::GEQUAL, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::Equal { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::EQUAL, ref_value as GLint, mask_value as GLuint));
            }
            StencilFunction::NotEqual { ref_value, mask_value } => {
                ffi!(gl::Enable(gl::STENCIL_TEST));
                ffi!(gl::StencilFunc(gl::NOTEQUAL, ref_value as GLint, mask_value as GLuint));
            }
        }
        gl_check_error();
    }
}


const MASK_VIEWPORT: u64 = 0x1;
const MASK_DEPTH: u64 = 0x2;
const MASK_STENCIL: u64 = 0x4;
const MASK_BLEND: u64 = 0x8;
const MASK_CULL: u64 = 0x10;
//const MASK_SCISOR : u64 = ;
//const MASK_BUFFERWRITE : u64 = ; // color and depth
//const MASK_ : u64 = ; // color and depth
const MASK_ALL: u64 = 0x1F;


/// Structure to handle gl states
#[derive(Debug)]
pub struct StateManager {
    forced: bool,
    prev_used_mask: u64,
    used_mask: u64,
    dirty_mask: u64,
    viewport: ViewportState,
    depth: DepthState,
    stencil: StencilState,
    blend: BlendState,
    cull: CullState,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            forced: false,
            prev_used_mask: 0,
            used_mask: 0,
            dirty_mask: MASK_ALL,
            viewport: ViewportState::new(),
            depth: DepthState { value: Default::default() },
            stencil: StencilState { value: Default::default() },
            blend: BlendState { value: Default::default() },
            cull: CullState { value: Default::default() },
        }
    }

    pub fn set_forced(&mut self, forced: bool) {
        self.forced = forced;
    }

    pub fn commit(&mut self) {
        // we have to reset the states those were changed in the previous render
        // but not in this one
        let reset = self.prev_used_mask & !self.used_mask;

        // remember our used states to reset them on the next render call
        // prev must be set before the reset calls as those would effect incorrectly the used_mask
        self.prev_used_mask = self.used_mask;

        // reset all the unused states
        if (reset & MASK_VIEWPORT) != 0 { self.set_viewport(Default::default()); }
        if (reset & MASK_DEPTH) != 0 { self.set_depth(Default::default()); }
        if (reset & MASK_STENCIL) != 0 { self.set_stencil(Default::default()); }
        if (reset & MASK_BLEND) != 0 { self.set_blend(Default::default()); }
        if (reset & MASK_CULL) != 0 { self.set_cull(Default::default()); }

        // apply state changes
        if self.forced { self.dirty_mask = MASK_ALL; }
        if self.dirty_mask == 0 { return; }
        if (self.dirty_mask & MASK_VIEWPORT) != 0 { self.viewport.commit(); }
        if (self.dirty_mask & MASK_DEPTH) != 0 { self.depth.commit(); }
        if (self.dirty_mask & MASK_STENCIL) != 0 { self.stencil.commit(); }
        if (self.dirty_mask & MASK_BLEND) != 0 { self.blend.commit(); }
        if (self.dirty_mask & MASK_CULL) != 0 { self.cull.commit(); }

        self.dirty_mask = 0;
        self.used_mask = 0;
    }

    pub fn set_render_size(&mut self, size: Size) {
        self.used_mask |= MASK_VIEWPORT;
        if self.viewport.render_size == size { return; }
        self.dirty_mask |= MASK_VIEWPORT;
        self.viewport.render_size = size;
    }

    pub fn set_viewport(&mut self, fun: Viewport) {
        self.used_mask |= MASK_VIEWPORT;
        if self.viewport.function == fun { return; }
        self.dirty_mask |= MASK_VIEWPORT;
        self.viewport.function = fun;
    }

    pub fn set_depth(&mut self, fun: DepthFunction) {
        self.used_mask |= MASK_DEPTH;
        if self.depth.value == fun { return; }
        self.dirty_mask |= MASK_DEPTH;
        self.depth.value = fun;
    }

    pub fn set_stencil(&mut self, fun: StencilFunction) {
        self.used_mask |= MASK_STENCIL;
        if self.stencil.value == fun { return; }
        self.dirty_mask |= MASK_STENCIL;
        self.stencil.value = fun;
    }

    pub fn set_blend(&mut self, fun: BlendFunction) {
        self.used_mask |= MASK_BLEND;
        if self.blend.value == fun { return; }
        self.dirty_mask |= MASK_BLEND;
        self.blend.value = fun;
    }

    pub fn set_cull(&mut self, fun: CullFunction) {
        self.used_mask |= MASK_CULL;
        if self.cull.value == fun { return; }
        self.dirty_mask |= MASK_CULL;
        self.cull.value = fun;
    }
}

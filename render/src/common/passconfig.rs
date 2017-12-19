#![allow(dead_code)]
#![deny(missing_docs)]

use common::*;


/// Module to store pass related objects.
/// Most of the render objects are visible in the render module directly, but it seems as
/// pass config require many small tweaks. So instead of polutiong the render module, a new
/// namespace is created.
pub mod pass {
    use super::*;

    /// Enum to store the clear policy
    #[derive(Copy, Clone, Debug)]
    pub enum Clear {
        /// No buffers are cleared.
        None,

        /// Clear color buffers with the argument
        Frame(Float32x3),
    }

    /// Enum to store the view port
    #[derive(Copy, Clone, Debug)]
    pub enum ViewPort {
        /// Ignore viewport settings.
        None,

        /// Use the full render target
        Fullscreen,

        /// Use the given rectangle given in pixels
        PixelRectangle(Rectangle),
    }
}

/// Settings structure for RenderPass behavior.
///
/// This structure stores everything that can be customized when
/// constructing a render pass.
#[derive(Copy, Clone, Debug)]
pub struct PassConfig {
    /// Viewport.
    pub view_port: pass::ViewPort,

    /// Clear policy.
    pub clear: pass::Clear,
}

impl PassConfig {
    /// Creates render pass  settings with defaults.
    pub fn new() -> PassConfig {
        PassConfig {
            view_port: pass::ViewPort::Fullscreen,
            clear: pass::Clear::None,
        }
    }

    /// Sets the clear color and returns Self for chained function calls.
    pub fn set_clear_color<C: Into<Float32x3>>(&mut self, clear_color: C) -> &mut PassConfig {
        self.clear = pass::Clear::Frame(clear_color.into());
        self
    }

    /// Sets the viewport to fullscreen
    pub fn set_fullscreen(&mut self) -> &mut PassConfig {
        self.view_port = pass::ViewPort::Fullscreen;
        self
    }
}
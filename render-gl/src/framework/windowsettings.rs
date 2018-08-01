use std::rc::Rc;
use std::cell::RefCell;

use core::*;
use framework::*;
use resources::*;

/// OpenGL profile selection.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpenGLProfile {
    /// No specific profile is requested
    DontCare,
    /// Core profile
    Core,
    /// Compatibility profile
    Compatibility,
    /// OpenGL ES profile
    ES2,
}

/// OpenGL driver robustness.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpenGLRobustness {
    /// No specific robustness is requested
    DontCare,
    /// TBD
    NoReset,
    /// TBD
    LoseContextOnReset,
}

/// OpenGL release behavior.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpenGLRelease {
    /// No specific release method is requested
    DontCare,
    /// TBD
    None,
    /// TBD
    Flush,
}


/// Extra platform dependent settings
#[derive(Debug, Copy, Clone)]
pub struct GLExtraWindowSettings {
    /// The selected a context version.
    ///
    /// This setting is valid only for OpenGL, other implementations ignore it.
    pub gl_version: (u8, u8),

    /// Indicates to remove "deprecated" functionality
    ///
    /// This setting is valid only for OpenGL for specific profiles, other implementations ignore it.
    pub gl_forward_compatible: bool,

    /// Selected the compatiblity profile, see https://www.khronos.org/opengl/wiki/Core_And_Compatibility_in_Contexts
    ///
    /// This setting is valid only for OpenGL, other implementations ignore it.
    pub gl_profile: OpenGLProfile,

    /// Selected robust mode of the driver
    ///
    /// This setting is valid only for OpenGL, other implementations ignore it.
    pub gl_robustness: OpenGLRobustness,

    /// Selected release mode
    ///
    /// This setting is valid only for OpenGL, other implementations ignore it.
    pub gl_release: OpenGLRelease,
}

impl GLExtraWindowSettings {
    /// Gets the selected context version.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_version(&self) -> (u8, u8) {
        self.gl_version
    }

    /// Sets the required context version in method chaining.
    ///
    /// Only request an explicitly versioned context when necessary, as explicitly requesting
    /// version 1.0 does not always return the highest version supported by the driver.
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_version(&mut self, major: u8, minor: u8) -> &mut Self {
        self.gl_version = (major, minor);
        self
    }

    /// Gets whether built windows should remove the "deprecate" functionality.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_forward_compatible(&self) -> bool {
        self.gl_forward_compatible
    }

    /// Sets whether built windows should remove the "deprecate" functionality in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_forward_compatible(&mut self, value: bool) -> &mut Self {
        self.gl_forward_compatible = value;
        self
    }

    /// Gets the selected API profile version.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_profile(&self) -> OpenGLProfile {
        self.gl_profile
    }

    /// Sets the selected API profile version in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_profile(&mut self, value: OpenGLProfile) -> &mut Self {
        self.gl_profile = value;
        self
    }

    /// Gets the selected OpenGl driver robustness.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_robustness(&self) -> OpenGLRobustness {
        self.gl_robustness
    }

    /// Selectes OpenGl driver robustness in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_robustness(&mut self, value: OpenGLRobustness) -> &mut Self {
        self.gl_robustness = value;
        self
    }

    /// Gets the selected OpenGl release method.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_release(&self) -> OpenGLRelease {
        self.gl_release
    }

    /// Selectes OpenGl release method in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_release(&mut self, value: OpenGLRelease) -> &mut Self {
        self.gl_release = value;
        self
    }
}

impl Default for GLExtraWindowSettings {
    /// Creates extra window settings with defaults.
   ///
   /// - gl_version: (0, 0)
   /// - gl_forward_compatible: false
   /// - gl_profile: OpenGLProfile::DontCare
   /// - gl_robustness: OpenGLRobustness::DontCare
   /// - gl_release: OpenGLRelease::DontCare,
    fn default() -> GLExtraWindowSettings {
        GLExtraWindowSettings {
            gl_version: (0, 0),
            gl_forward_compatible: false,
            gl_profile: OpenGLProfile::DontCare,
            gl_robustness: OpenGLRobustness::DontCare,
            gl_release: OpenGLRelease::DontCare,
        }
    }
}

/// WindowSettings implementation for opengl.
pub type PlatformWindowSettings = WindowSettings<GLExtraWindowSettings>;

impl PlatformWindowBuilder for WindowSettings<GLExtraWindowSettings> {
    /// Builds window from the given settings.
    ///
    /// # Errors
    ///
    /// This function will return an error if the backend cannot create the window
    fn build<V: View<Resources=GLResources>>(&self, engine: &PlatformEngine, view: V) -> Result<PlatformWindow, Error> {
        let view = Rc::new(RefCell::new(view));
        GLWindow::new_boxed(self, engine.platform(), view)
    }
}

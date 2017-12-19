#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::fmt::Debug;
use engine::*;

/// Constant indicating a "don't" care for the render surface requirements
pub const FBCONFIG_DONT_CARE: u8 = 255;

/// Settings structure for render surface requirement
#[derive(Debug, Copy, Clone)]
pub struct FBConfig {
    /// Request a specific pixel format by id, or 0
    pub handle: u32,

    /// Required number of red bits or FBCONFIG_DONT_CARE
    pub red_bits: u8,
    /// Required number of green bits or FBCONFIG_DONT_CARE
    pub green_bits: u8,
    /// Required number of blue bits or FBCONFIG_DONT_CARE
    pub blue_bits: u8,
    /// Required number of alpha bits or FBCONFIG_DONT_CARE
    pub alpha_bits: u8,
    /// Required number of depth bits or FBCONFIG_DONT_CARE
    pub depth_bits: u8,
    /// Required number of stencil bits or FBCONFIG_DONT_CARE
    pub stencil_bits: u8,
    /// Required number of red bits in the accumulation buffer or FBCONFIG_DONT_CARE
    pub accum_red_bits: u8,
    /// Required number of green bits in the accumulation buffer or FBCONFIG_DONT_CARE
    pub accum_green_bits: u8,
    /// Required number of blue bits in the accumulation buffer or FBCONFIG_DONT_CARE
    pub accum_blue_bits: u8,
    /// Required number of alpha bits in the accumulation buffer or FBCONFIG_DONT_CARE
    pub accum_alpha_bits: u8,
    /// Required number of auxilary buffers or FBCONFIG_DONT_CARE
    pub aux_buffers: u8,
    /// Required number of sub-samples or FBCONFIG_DONT_CARE
    pub samples: u8,
    /// Require stereo rendering or false
    pub stereo: bool,
    /// Require double buffering or false
    pub double_buffer: bool,
    /// Require hardware accelerated color conversion or false
    pub srgb: bool,

    /// Enable vertical sync
    pub vsync: bool,
    /// Enable debugging at driver level, if supported
    pub debug: bool,
}


/// Settings structure for window behavior.
///
/// This structure stores everything that can be customized when
/// constructing most windows.
#[derive(Clone)]
pub struct WindowSettings<E: Copy + Debug + Default> {
    pub ( crate )title: String,
    pub ( crate )size: Size,
    pub ( crate )sub_samples: u8,
    pub ( crate )fullscreen: bool,
    pub ( crate )resizable: bool,
    pub ( crate )decorated: bool,
    pub ( crate )fb_config: FBConfig,
    pub ( crate )platform_extra: E,

}

impl<E: Copy + Debug + Default> Default for WindowSettings<E> {
    /// Creates window settings with defaults.
    ///
    /// - sub_samples: 0
    /// - fullscreen: false
    /// - resizable: true
    /// - decorated: true
    /// - handle: 0
    ///
    /// - red_bits: FBCONFIG_DONT_CARE
    /// - green_bits: FBCONFIG_DONT_CARE
    /// - blue_bits: FBCONFIG_DONT_CARE
    /// - alpha_bits: FBCONFIG_DONT_CARE
    /// - depth_bits: FBCONFIG_DONT_CARE
    /// - stencil_bits: FBCONFIG_DONT_CARE
    /// - accum_red_bits: FBCONFIG_DONT_CARE
    /// - accum_green_bits: FBCONFIG_DONT_CARE
    /// - accum_blue_bits: FBCONFIG_DONT_CARE
    /// - accum_alpha_bits: FBCONFIG_DONT_CARE
    /// - aux_buffers: FBCONFIG_DONT_CARE
    /// - samples: FBCONFIG_DONT_CARE
    ///
    /// - stereo: false
    /// - double_buffer: true
    /// - srgb: false
    /// - vsync: true
    /// - debug: false
    ///
    fn default() -> WindowSettings<E> {
        WindowSettings {
            title: "hello".into(),
            size: Size { width: 640, height: 480 },
            sub_samples: 0,
            fullscreen: false,
            resizable: true,
            decorated: true,
            fb_config: FBConfig {
                handle: 0,
                red_bits: FBCONFIG_DONT_CARE,
                green_bits: FBCONFIG_DONT_CARE,
                blue_bits: FBCONFIG_DONT_CARE,
                alpha_bits: FBCONFIG_DONT_CARE,
                depth_bits: FBCONFIG_DONT_CARE,
                stencil_bits: FBCONFIG_DONT_CARE,
                accum_red_bits: FBCONFIG_DONT_CARE,
                accum_green_bits: FBCONFIG_DONT_CARE,
                accum_blue_bits: FBCONFIG_DONT_CARE,
                accum_alpha_bits: FBCONFIG_DONT_CARE,
                aux_buffers: FBCONFIG_DONT_CARE,
                samples: FBCONFIG_DONT_CARE,
                stereo: false,
                double_buffer: true,
                srgb: false,

                vsync: true,
                debug: false,
            },
            platform_extra: Default::default(),
        }
    }
}

impl<E: Copy + Debug + Default> WindowSettings<E> {
    /// Gets the title of built windows.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Sets the title of built windows in method chaining.
    pub fn title<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.title = value.into();
        self
    }

    /// Gets the size of built windows.
    pub fn get_size(&self) -> Size {
        self.size
    }

    /// Sets the size of built windows in method chaining.
    pub fn size<S: Into<Size>>(&mut self, value: S) -> &mut Self {
        self.size = value.into();
        self
    }

    /// Gets whether built windows will be fullscreen.
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Sets whether built windows will be fullscreen in method chaining.
    pub fn fullscreen(&mut self, value: bool) -> &mut Self {
        self.fullscreen = value;
        self
    }

    /// Gets whether built windows should be resizable.
    pub fn get_resizable(&self) -> bool {
        self.resizable
    }

    /// Sets whether built windows should be resizable in method chaining.
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.resizable = value;
        self
    }

    /// Gets whether built windows should be decorated by the OS.
    pub fn get_decorated(&self) -> bool {
        self.decorated
    }

    /// Sets whether built windows should be decorated by the OS in method chaining.
    pub fn decorated(&mut self, value: bool) -> &mut Self {
        self.decorated = value;
        self
    }

    /// Gets the explicitly selected pixel format id (OS dependent).
    pub fn get_fb_handle(&self) -> u32 {
        self.fb_config.handle
    }

    /// Sets the explicitly selected pixel format id (OS dependent) in method chaining.
    pub fn fb_handle(&mut self, value: u32) -> &mut Self {
        self.fb_config.handle = value;
        self
    }

    /// Gets the requested number of bits in the color buffer.
    pub fn get_fb_color_bits(&self) -> (u8, u8, u8, u8) {
        (self.fb_config.red_bits, self.fb_config.green_bits, self.fb_config.blue_bits, self.fb_config.alpha_bits)
    }

    /// Sets the requested number of bits in the color buffer.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn fb_color_bits(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.fb_config.red_bits = r;
        self.fb_config.green_bits = g;
        self.fb_config.blue_bits = b;
        self.fb_config.alpha_bits = a;
        self
    }

    /// Gets the requested number of bits in the depth and stencil buffers.
    pub fn get_fb_depth_bits(&self) -> (u8, u8) {
        (self.fb_config.depth_bits, self.fb_config.stencil_bits)
    }

    /// Sets the requested number of bits in the depth and stencil buffers in method chaining.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn fb_depth_bits(&mut self, depth: u8, stencil: u8) -> &mut Self {
        self.fb_config.depth_bits = depth;
        self.fb_config.stencil_bits = stencil;
        self
    }

    /*
    accum_red_bits: FBCONFIG_DONT_CARE,
    accum_green_bits: FBCONFIG_DONT_CARE,
    accum_blue_bits: FBCONFIG_DONT_CARE,
    accum_alpha_bits: FBCONFIG_DONT_CARE,

    aux_buffers: FBCONFIG_DONT_CARE,
    */

    /// Gets whether built windows should use stereo rendering.
    pub fn get_fb_stereo(&self) -> bool {
        self.fb_config.stereo
    }

    /// Sets whether built windows should use stereo rendering in method chaining.
    pub fn fb_stereo(&mut self, value: bool) -> &mut Self {
        self.fb_config.stereo = value;
        self
    }

    /// Gets whether built windows should use double buffering.
    pub fn get_fb_double_buffer(&self) -> bool {
        self.fb_config.double_buffer
    }

    /// Sets whether built windows should use double buffering in method chaining.
    pub fn fb_double_buffer(&mut self, value: bool) -> &mut Self {
        self.fb_config.double_buffer = value;
        self
    }

    /// Gets the number of samples to use for anti-aliasing.
    pub fn get_fb_sub_samples(&self) -> u8 {
        self.fb_config.samples
    }

    /// Sets the number of samples to use for anti-aliasing in method chaining.
    pub fn fb_sub_samples(&mut self, value: u8) -> &mut Self {
        self.fb_config.samples = value;
        self
    }

    /// Gets whether built windows should use hardware accelerated color conversion.
    pub fn get_fb_srgb(&self) -> bool {
        self.fb_config.srgb
    }

    /// Sets whether built windows should use hardware accelerated color conversion in method chaining.
    pub fn fb_srgb(&mut self, value: bool) -> &mut Self {
        self.fb_config.srgb = value;
        self
    }

    /// Gets whether built windows should use vsync.
    pub fn get_fb_vsync(&self) -> bool {
        self.fb_config.vsync
    }

    /// Sets whether built windows should use vsync in method chaining.
    pub fn fb_vsync(&mut self, value: bool) -> &mut Self {
        self.fb_config.vsync = value;
        self
    }

    /// Gets whether built windows should enable driver level debugging.
    pub fn get_fb_debug(&self) -> bool {
        self.fb_config.debug
    }

    /// Sets whether built windows should enable driver level debugging in method chaining.
    pub fn fb_debug(&mut self, value: bool) -> &mut Self {
        self.fb_config.debug = value;
        self
    }

    /// Gets platform specific extra parameters
    pub fn get_extra(&self) -> &E {
        &self.platform_extra
    }

    /// Gets platform specific extra parameters
    pub fn extra<F: FnOnce(&mut E)>(&mut self, f: F) -> &mut Self {
        f(&mut self.platform_extra);
        self
    }
}

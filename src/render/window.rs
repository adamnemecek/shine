#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;


/// Enum to store the error occurred during a call to a render function.
#[derive(Debug, Clone)]
pub enum Error {
    /// Error reported during a window creation.
    CreationError(String),
    /// Error reported by the OS during rendering
    ContextError(String),
    /// Context is lost, ex window has been closed.
    ContextLost,
}


/// Structure to store the window size.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    /// The width.
    pub width: u32,
    /// The height.
    pub height: u32,
}

impl From<[u32; 2]> for Size {
    #[inline(always)]
    fn from(value: [u32; 2]) -> Size {
        Size {
            width: value[0],
            height: value[1],
        }
    }
}

impl From<(u32, u32)> for Size {
    #[inline(always)]
    fn from(value: (u32, u32)) -> Size {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<Size> for [u32; 2] {
    #[inline(always)]
    fn from(value: Size) -> [u32; 2] {
        [value.width, value.height]
    }
}

impl From<Size> for (u32, u32) {
    #[inline(always)]
    fn from(value: Size) -> (u32, u32) {
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


/// Enum to store the window events.
pub trait SurfaceEventHandler: 'static {
    /// Handles to surface lost event.
    ///
    /// Window still has the OS resources, but will be released soon after this call.
    fn on_lost(&mut self, &mut Window);

    /// Handles to surface ready event.
    ///
    /// Window has create all the OS resources.
    fn on_ready(&mut self, &mut Window);
}


/// Constant indicating a "don't" care for the render surface requirements
pub const FBCONFIG_DONT_CARE: u8 = 255;

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


/// Settings structure for window behavior.
///
/// This structure stores everything that can be customized when
/// constructing most windows.
#[derive(Clone)]
pub struct WindowSettings {
    /// Title of the window
    pub title: String,
    /// Size of the window
    pub size: Size,
    /// Sub-sampling
    pub sub_samples: u8,
    /// Enable fullscreen
    pub fullscreen: bool,
    /// Enable resizing of the window
    pub resizable: bool,
    /// Enable the OS to decorate of the window
    pub decorated: bool,
    /// Rander surface requirements
    pub fb_config: FBConfig,
}

impl WindowSettings {
    /// Creates window settings with defaults.
    ///
    /// - sub_samples: 0
    /// - fullscreen: false
    /// - vsync: false
    /// - srgb: true
    /// - resizable: true
    /// - decorated: true
    /// - controllers: true
    pub fn new() -> WindowSettings {
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

                gl_version: (0, 0),
                gl_forward_compatible: false,
                gl_profile: OpenGLProfile::DontCare,
                gl_robustness: OpenGLRobustness::DontCare,
                gl_release: OpenGLRelease::DontCare,
            },
        }
    }

    /// Builds window from the given settings.
    ///
    /// # Errors
    ///
    /// This function will return an error if thc current backend returns an error.
    pub fn build(self, engine: &mut Engine) -> Result<Window, Error> {
        Window::new(self, engine)
    }

    /// Gets the title of built windows.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Sets the title of built windows.
    pub fn set_title<T: Into<String>>(&mut self, value: T) {
        self.title = value.into();
    }

    /// Sets the title of built windows in method chaining.
    pub fn title<T: Into<String>>(mut self, value: T) -> Self {
        self.set_title(value);
        self
    }

    /// Gets the size of built windows.
    pub fn get_size(&self) -> Size {
        self.size
    }

    /// Sets the size of built windows.
    pub fn set_size<S: Into<Size>>(&mut self, value: S) {
        self.size = value.into();
    }

    /// Sets the size of built windows in method chaining.
    pub fn size<S: Into<Size>>(mut self, value: S) -> Self {
        self.set_size(value);
        self
    }

    /// Gets whether built windows will be fullscreen.
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Sets whether built windows will be fullscreen.
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }

    /// Sets whether built windows will be fullscreen in method chaining.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.set_fullscreen(value);
        self
    }

    /// Gets whether built windows should be resizable.
    pub fn get_resizable(&self) -> bool {
        self.resizable
    }

    /// Sets whether built windows should be resizable.
    pub fn set_resizable(&mut self, value: bool) {
        self.resizable = value;
    }

    /// Sets whether built windows should be resizable in method chaining.
    pub fn resizable(mut self, value: bool) -> Self {
        self.set_resizable(value);
        self
    }

    /// Gets whether built windows should be decorated by the OS.
    pub fn get_decorated(&self) -> bool {
        self.decorated
    }

    /// Sets whether built windows should be decorated by the OS.
    pub fn set_decorated(&mut self, value: bool) {
        self.decorated = value;
    }

    /// Sets whether built windows should be decorated by the OS in method chaining.
    pub fn decorated(mut self, value: bool) -> Self {
        self.set_decorated(value);
        self
    }

    /// Gets the explicitly selected pixel format id (OS dependent).
    pub fn get_fb_handle(&self) -> u32 {
        self.fb_config.handle
    }

    /// Sets the explicitly selected pixel format id (OS dependent).
    pub fn set_fb_handle(&mut self, value: u32) {
        self.fb_config.handle = value;
    }

    /// Sets the explicitly selected pixel format id (OS dependent) in method chaining.
    pub fn fb_handle(mut self, value: u32) -> Self {
        self.set_fb_handle(value);
        self
    }

    /// Gets the requested number of bits in the color buffer.
    pub fn get_fb_color_bits(&self) -> (u8, u8, u8, u8) {
        (self.fb_config.red_bits, self.fb_config.green_bits, self.fb_config.blue_bits, self.fb_config.alpha_bits)
    }

    /// Sets the requested number of bits in the color buffer.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn set_fb_color_bits(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.fb_config.red_bits = r;
        self.fb_config.green_bits = g;
        self.fb_config.blue_bits = b;
        self.fb_config.alpha_bits = a;
    }

    /// Sets the requested number of bits in the color buffer.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn fb_color_bits(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.set_fb_color_bits(r, g, b, a);
        self
    }

    /// Gets the requested number of bits in the depth and stencil buffers.
    pub fn get_fb_depth_bits(&self) -> (u8, u8) {
        (self.fb_config.depth_bits, self.fb_config.stencil_bits)
    }

    /// Sets the requested number of bits in the depth and stencil buffers.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn set_fb_depth_bits(&mut self, depth: u8, stencil: u8) {
        self.fb_config.depth_bits = depth;
        self.fb_config.stencil_bits = stencil;
    }

    /// Sets the requested number of bits in the depth and stencil buffers in method chaining.
    ///
    /// If no constraint is required, set the ignored components to FBCONFIG_DONT_CARE.
    pub fn fb_depth_bits(mut self, depth: u8, stencil: u8) -> Self {
        self.set_fb_depth_bits(depth, stencil);
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

    /// Sets whether built windows should use double buffering.
    pub fn set_fb_stereo(&mut self, value: bool) {
        self.fb_config.stereo = value;
    }

    /// Sets whether built windows should use stereo rendering in method chaining.
    pub fn fb_stereo(mut self, value: bool) -> Self {
        self.set_fb_stereo(value);
        self
    }

    /// Gets whether built windows should use double buffering.
    pub fn get_fb_double_buffer(&self) -> bool {
        self.fb_config.double_buffer
    }

    /// Sets whether built windows should use double buffering.
    pub fn set_fb_double_buffer(&mut self, value: bool) {
        self.fb_config.double_buffer = value;
    }

    /// Sets whether built windows should use double buffering in method chaining.
    pub fn fb_double_buffer(mut self, value: bool) -> Self {
        self.set_fb_double_buffer(value);
        self
    }

    /// Gets the number of samples to use for anti-aliasing.
    pub fn get_fb_sub_samples(&self) -> u8 {
        self.fb_config.samples
    }

    /// Sets the number of samples to use for anti-aliasing.
    pub fn set_fb_sub_samples(&mut self, value: u8) {
        self.fb_config.samples = value;
    }

    /// Sets the number of samples to use for anti-aliasing in method chaining.
    pub fn fb_sub_samples(mut self, value: u8) -> Self {
        self.set_fb_sub_samples(value);
        self
    }

    /// Gets whether built windows should use hardware accelerated color conversion.
    pub fn get_fb_srgb(&self) -> bool {
        self.fb_config.srgb
    }

    /// Sets whether built windows should use hardware accelerated color conversion.
    pub fn set_fb_srgb(&mut self, value: bool) {
        self.fb_config.srgb = value;
    }

    /// Sets whether built windows should use hardware accelerated color conversion in method chaining.
    pub fn fb_srgb(mut self, value: bool) -> Self {
        self.set_fb_srgb(value);
        self
    }

    /// Gets whether built windows should use vsync.
    pub fn get_fb_vsync(&self) -> bool {
        self.fb_config.vsync
    }

    /// Sets whether built windows should use vsync.
    pub fn set_fb_vsync(&mut self, value: bool) {
        self.fb_config.vsync = value;
    }

    /// Sets whether built windows should use vsync in method chaining.
    pub fn fb_vsync(mut self, value: bool) -> Self {
        self.set_fb_vsync(value);
        self
    }

    /// Gets whether built windows should enable driver level debugging.
    pub fn get_fb_debug(&self) -> bool {
        self.fb_config.debug
    }

    /// Sets whether built windows should enable driver level debugging.
    pub fn set_fb_debug(&mut self, value: bool) {
        self.fb_config.debug = value;
    }

    /// Sets whether built windows should enable driver level debugging in method chaining.
    pub fn fb_debug(mut self, value: bool) -> Self {
        self.set_fb_debug(value);
        self
    }


    /// Gets the selected context version.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_version(&self) -> (u8, u8) {
        self.fb_config.gl_version
    }

    /// Sets the required context version.
    ///
    /// Only request an explicitly versioned context when necessary, as explicitly requesting
    /// version 1.0 does not always return the highest version supported by the driver.
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn set_gl_version(&mut self, major: u8, minor: u8) {
        self.fb_config.gl_version = (major, minor);
    }

    /// Sets the required context version in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_version(mut self, major: u8, minor: u8) -> Self {
        self.set_gl_version(major, minor);
        self
    }

    /// Gets whether built windows should remove the "deprecate" functionality.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_forward_compatible(&self) -> bool {
        self.fb_config.gl_forward_compatible
    }

    /// Sets whether built windows should remove the "deprecate" functionality.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn set_gl_forward_compatible(&mut self, value: bool) {
        self.fb_config.gl_forward_compatible = value;
    }

    /// Sets whether built windows should remove the "deprecate" functionality in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_forward_compatible(mut self, value: bool) -> Self {
        self.set_gl_forward_compatible(value);
        self
    }

    /// Gets the selected API profile version.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_gl_profile(&self) -> OpenGLProfile {
        self.fb_config.gl_profile
    }

    /// Sets the selected API profile version.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn set_gl_profile(&mut self, value: OpenGLProfile) {
        self.fb_config.gl_profile = value;
    }

    /// Sets the selected API profile version in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn gl_profile(mut self, value: OpenGLProfile) -> Self {
        self.set_gl_profile(value);
        self
    }

    /// Gets the selected OpenGl driver robustness.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_fb_gl_robustness(&self) -> OpenGLRobustness {
        self.fb_config.gl_robustness
    }

    /// Selectes OpenGl driver robustness.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn set_fb_gl_robustness(&mut self, value: OpenGLRobustness) {
        self.fb_config.gl_robustness = value;
    }

    /// Selectes OpenGl driver robustness in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn fb_gl_robustness(mut self, value: OpenGLRobustness) -> Self {
        self.set_fb_gl_robustness(value);
        self
    }

    /// Gets the selected OpenGl release method.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn get_fb_gl_release(&self) -> OpenGLRelease {
        self.fb_config.gl_release
    }

    /// Selectes OpenGl release method.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn set_fb_gl_release(&mut self, value: OpenGLRelease) {
        self.fb_config.gl_release = value;
    }

    /// Selectes OpenGl release method in method chaining.
    ///
    /// This setting is valid for OpenGL context and ignored by other engine implementations.
    pub fn fb_gl_release(mut self, value: OpenGLRelease) -> Self {
        self.set_fb_gl_release(value);
        self
    }
}


/// Structure to store the window abstraction.
///
/// The structure stores the platform dependent implementation and serves as a bridge between
/// the abstraction and the concrete implementation.
pub struct Window {
    /// Stores the platform dependent implementation.
    pub platform: WindowImpl
}

impl Window {
    // Creates a new window with the given settings.
    ///
    /// # Error
    ///
    /// This function will return an error if the current backend cannot create the
    /// window.
    pub fn new(settings: WindowSettings, engine: &mut Engine) -> Result<Window, Error> {
        let platform = try!(WindowImpl::new(settings, engine));
        Ok(Window { platform: platform })
    }

    /// Sets the surface event handler.
    ///
    /// Event handler can be altered any time, but it is preferred to set them before
    /// the show call no to miss the on_ready event.
    pub fn set_surface_handler<H: SurfaceEventHandler>(&mut self, handler: H) {
        self.platform.set_surface_handler(handler);
    }

    /// Starts the closing process.
    ///
    /// This function asks the OS to close the window. Window is not closed immediately,
    /// event handling shall continue the execution until the OS close events arrive.
    pub fn close(&mut self) {
        self.platform.close()
    }

    /// Returns true if the window is closed or in closing state.
    pub fn is_closed(&self) -> bool {
        self.platform.is_closed()
    }

    /// Gets the size of the window.
    pub fn size(&self) -> Size {
        self.platform.size()
    }

    /// Gets the size of the draw area of the window.
    pub fn draw_size(&self) -> Size {
        self.platform.draw_size()
    }

    /// Prepares the window for rendering.
    pub fn start_render(&self) -> Result<(), Error> {
        self.platform.start_render()
    }

    /// Sends a command queue for rendering.
    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), Error> {
        self.platform.process_queue(queue)
    }

    /// Swaps the buffers and perform post render tasks.
    pub fn end_render(&self) -> Result<(), Error> {
        self.platform.end_render()
    }

    /// Renders a single que.
    ///
    /// This function is a shortcut for star, process, end cycle.
    pub fn render_single_queue(&self, queue: &mut CommandQueue) -> Result<(), Error> {
        try!(self.start_render());
        println!("ab");
        try!(self.process_queue(queue));
        println!("ac");
        try!(self.end_render());
        println!("ad");
        Ok(())
    }
}

impl From<WindowImpl> for Window {
    #[inline(always)]
    fn from(value: WindowImpl) -> Window {
        Window {
            platform: value,
        }
    }
}

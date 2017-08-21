#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;


/// Enum to store the error occurred during a window creation.
#[derive(Debug, Clone)]
pub enum CreationError {
    /// Some error reported by the OS
    OsError(String),
    /// Engine is not initialized error. Call Engine::init prior using this functionality
    EngineNotInitialized,
    /// Some required is not implemented yet
    NotImplemented(String),
}


/// Enum to store the error occurred during a call to a render function.
#[derive(Debug, Copy, Clone)]
pub enum ContextError {
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
    /// Enable vsync
    pub vsync: bool,
    /// Enable hardware accelerated color conversion.
    pub srgb: bool,
    /// Enable resizing of the window
    pub resizable: bool,
    /// Enable the OS to decorate of the window
    pub decorated: bool,
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
            vsync: false,
            srgb: true,
            resizable: true,
            decorated: true,
        }
    }

    /// Builds window from the given settings.
    ///
    /// # Errors
    ///
    /// This function will return an error if thc current backend returns an error.
    pub fn build(self, engine: &mut Engine) -> Result<Window, CreationError> {
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

    /// Gets the number of samples to use for anti-aliasing.
    pub fn get_sub_samples(&self) -> u8 {
        self.sub_samples
    }

    /// Sets the number of samples to use for anti-aliasing.
    pub fn set_sub_samples(&mut self, value: u8) {
        self.sub_samples = value;
    }

    /// Sets the number of samples to use for anti-aliasing in method chaining.
    pub fn samples(mut self, value: u8) -> Self {
        self.set_sub_samples(value);
        self
    }

    /// Gets whether built windows should use vsync.
    pub fn get_vsync(&self) -> bool {
        self.vsync
    }

    /// Sets whether built windows should use vsync.
    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    /// Sets whether built windows should use vsync in method chaining.
    pub fn vsync(mut self, value: bool) -> Self {
        self.set_vsync(value);
        self
    }

    /// Gets whether built windows should use hardware accelerated color conversion.
    pub fn get_srgb(&self) -> bool {
        self.srgb
    }

    /// Sets whether built windows should use hardware accelerated color conversion.
    pub fn set_srgb(&mut self, value: bool) {
        self.srgb = value;
    }

    /// Sets whether built windows should use hardware accelerated color conversion in method chaining.
    pub fn srgb(mut self, value: bool) -> Self {
        self.set_srgb(value);
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
    pub fn new(settings: WindowSettings, engine: &mut Engine) -> Result<Window, CreationError> {
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
    pub fn start_render(&self) -> Result<(), ContextError> {
        self.platform.start_render()
    }

    /// Sends a command queue for rendering.
    pub fn process_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
        self.platform.process_queue(queue)
    }

    /// Swaps the buffers and perform post render tasks.
    pub fn end_render(&self) -> Result<(), ContextError> {
        self.platform.end_render()
    }

    /// Renders a single que.
    ///
    /// This function is a shortcut for star, process, end cycle.
    pub fn render_single_queue(&self, queue: &mut CommandQueue) -> Result<(), ContextError> {
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

#![deny(missing_docs)]

use render::*;

/// Enum to store the error occurred during a window creation.
#[derive(Debug, Clone)]
pub enum EngineError {
    /// Engine could not be initialized error.
    /// The exact (OS) error message is also provided in the argument
     InitializeError(String),
}

/// Structure to store the engine abstraction.
pub struct Engine;

impl Engine {
    /// Initializes the engine.
    pub fn init() -> Result<(), EngineError> {
        EngineImpl::init()
    }

    /// Returns if engine was initialized.
    pub fn is_initialzed() -> bool {
        EngineImpl::is_initialzed()
    }

    /// Shuts down the engine.
    pub fn shutdown() {
        EngineImpl::shutdown();
    }
}

#![deny(missing_docs)]

use resources::*;

/// Trait that defined tha backend for the passes.
pub trait RenderTarget: Resource {
    /// Reference to this render target used in render passes.
    type Ref: Clone;

    /// Configures for the pass, called when configuration changes
    fn configure<Q: CommandQueue>(&self, queue: &Q, cfg: RenderTargetConfig);

    /// Prepare for rendering, called in each frame
    fn prepare<Q: CommandQueue>(&self, queue: &Q);
}

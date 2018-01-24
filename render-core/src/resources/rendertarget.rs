#![deny(missing_docs)]

use framework::*;
use resources::*;

/// Trait that defined tha backend for the passes.
pub trait RenderTarget<E: Engine>: Resource<E> {
    /// Reference to this render target used in render passes.
    type Ref: Clone;

    /// Configures for the pass, called when configuration changes
    fn configure(&self, queue: &mut E::FrameCompose, cfg: RenderTargetConfig);

    /// Prepare for rendering, called in each frame
    fn prepare(&self, queue: &mut E::FrameCompose);
}

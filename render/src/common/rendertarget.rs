use common::*;

/// Trait that defined tha backend for the passes.
pub trait RenderTarget: Resource {
    type Ref: Clone;

    fn new<R: ResourceManager>(resources: &R) -> Self where Self: Sized;

    /// Configures for the pass, called when configuration changes
    fn configure<Q: CommandQueue>(&self, queue: &Q, cfg: PassConfig);

    /// Prepare for rendering, called in each frame
    fn prepare<Q: CommandQueue>(&self, queue: &Q);
}

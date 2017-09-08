#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;

/// Geometry primitive type
#[derive(Copy, Clone, Debug)]
pub enum Primitive {
    /// Point primitive (1 vertex per primitive)
    Point,
    /// Line primitive (2 vertex per primitive)
    Line,
    /// Triangle primitive (3 vertex per primitive)
    Triangle,
}

pub (super) type PassIndex = u8;


/// Structure to store the render pass abstraction.
pub struct RenderPass {
    /// Stores the platform dependent implementation.
    pub platform: RenderPassImpl,

    index: PassIndex,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(index: PassIndex) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            index: index,
        }
    }

    /// Sets the viewport, size of the render target
    pub fn set_viewport(&mut self, size: Size) {
        self.platform.set_viewport(size);
    }

    /// Clears the render target
    pub fn clear(&mut self, t: f32) {
        self.platform.clear(t);
    }

    /// Submits a geometry for rendering
    pub fn draw(&mut self, queue: &mut CommandQueue, vertices: &VertexBuffer, primitive: Primitive, start: usize, vertex_count: usize) {
        self.platform.draw(queue, &vertices.platform, primitive, start, vertex_count);
    }
}

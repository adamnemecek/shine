#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::ops::DerefMut;

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
    platform: RenderPassImpl,

    command_store: Rc<RefCell<CommandStoreImpl>>,
    index: PassIndex,
}

impl RenderPass {
    /// Creates an empty shader.
    pub fn new(index: PassIndex, command_store: Rc<RefCell<CommandStoreImpl>>) -> RenderPass {
        RenderPass {
            platform: RenderPassImpl::new(),
            command_store: command_store,
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

    /// Gets the content of a render target
    /// pub fn get_texture(&mut self) -> Texture {}

    /// Submits a geometry for rendering
    pub fn draw(&mut self, queue: &mut CommandQueue, vertices: &VertexBuffer, primitive: Primitive, start: usize, vertex_count: usize) {
        self.platform.draw(queue.get_command_store().deref_mut(), &vertices.platform, primitive, start, vertex_count);
    }
}

impl CommandQueue for RenderPass {
    /// Returns the command queue
    fn get_command_store(&self) -> RefMut<CommandStoreImpl> {
        self.command_store.borrow_mut()
    }
}


#![deny(missing_docs)]

use std::slice;
use types::*;
use framework::*;
use resources::*;


/// Enum defining the type of shader source
#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    /// Vertex shader
    VertexShader,
    /// Fragment (pixel) shader
    FragmentShader,
}


/// Trait to store shader parameters.
/// It stores both the attributes and other shader parameters.
pub trait ShaderParameters<E: Engine>: Clone {
    /// Returns the number of attributes
    fn get_count() -> usize;

    /// Returns the index by attribute name
    fn get_index_by_name(name: &str) -> Option<usize>;

    /// Prepare parameters for rendering.
    fn bind(&self, context: &mut <E::Backend as Backend>::CommandContext);
}


/// Trait to define shader attribute and uniform names
pub trait ShaderDeclaration<E: Engine>: 'static + Clone {
    /// The structure storing the shader parameters.
    type Parameters: ShaderParameters<E>;

    /// Returns an iterator over the shader sources
    fn source_iter() -> slice::Iter<'static, (ShaderType, &'static str)>;
}


/// Structure to store the shader abstraction.
pub trait ShaderProgram<DECL: ShaderDeclaration<E>, E: Engine>: Resource<E> {
    /// Uploads and compiles the shader.
    fn compile(&self, queue: &mut E::CommandQueue);

    /// Resets self to a new handle and compiles the shader.
    /// If handle pointed to an existing resource prior this call, that resource is not modified, Backend will
    /// garbage collect it depending on the reference count.
    fn create_and_compile(&mut self, queue: &mut E::CommandQueue) {
        self.create(queue);
        self.compile(queue);
    }

    /// Sends a geometry for rendering
    fn draw(&self, queue: &mut E::CommandQueue, parameters: DECL::Parameters,
            primitive: Primitive, vertex_start: usize, vertex_count: usize);
}

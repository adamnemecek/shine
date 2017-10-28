#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;
use std::slice;

use render::*;

/// Enum defining the type of shader source
#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    /// Vertex shader
    VertexShader,
    /// Fragment (pixel) shader
    FragmentShader,
}

/// Trait to store vertex attribute parameters.
pub trait ShaderAttribute: Clone {
    /// Returns the number of attributes
    fn get_count() -> usize;

    /// Returns the index by attribute name
    fn get_index_by_name(name: &str) -> Option<usize>;

    /// Returns vertex attribute by index
    fn get_by_index(&self, index: usize) -> &VertexAttributeRefImpl;
}


/// Trait for the mutable visitor to process render types
#[allow(missing_docs)]
pub trait ShaderUniformVisitor {
    fn process_f32x16(&mut self, data: &Float32x16);
    fn process_f32x4(&mut self, data: &Float32x4);
    fn process_f32x3(&mut self, data: &Float32x3);
    fn process_f32x2(&mut self, data: &Float32x2);
    fn process_f32(&mut self, data: f32);

    fn process_tex_2d(&mut self, data: &Texture2DRefImpl);
}


/// Trait to store shader parameters.
pub trait ShaderUniform: Clone {
    /// Returns the number of uniforms
    fn get_count() -> usize;

    /// Returns the index by uniform name
    fn get_index_by_name(name: &str) -> Option<usize>;

    /// Visit data by index
    fn process_by_index<V: ShaderUniformVisitor>(&self, index: usize, visitor: &mut V);
}


/// Trait to define shader attribute and uniform names
pub trait ShaderDeclaration: 'static {
    /// The structure storing the vertex attribute parameters.
    type Attributes: ShaderAttribute;
    /// The structure storing the shader parameters.
    type Uniforms: ShaderUniform;

    /// Returns an iterator over the shader sources
    fn get_sources() -> slice::Iter<'static, (ShaderType, &'static str)>;
}



/// Structure to store the shader abstraction.
pub struct ShaderProgram<SD: ShaderDeclaration> {
    pub ( crate ) platform: ShaderProgramImpl,
    phantom: PhantomData<SD>,
}

impl<SD: ShaderDeclaration> ShaderProgram<SD> {
    /// Creates an empty shader.
    pub fn new() -> ShaderProgram<SD> {
        ShaderProgram {
            platform: ShaderProgramImpl::new(),
            phantom: PhantomData
        }
    }

    /// Releases the hw resources of a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }

    /// Compiles the shader.
    pub fn compile<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.compile::<SD, Q>(queue);
    }

    /// Sends a geometry for rendering
    pub fn draw<Q: CommandQueue>(&mut self, queue: &mut Q,
                                 attributes: SD::Attributes, uniforms: SD::Uniforms,
                                 primitive: Primitive, vertex_start: usize, vertex_count: usize)
    {
        self.platform.draw::<SD, Q>(queue, attributes, uniforms, primitive, vertex_start, vertex_count);
    }

    /// Sends a geometry for rendering
    pub fn draw_indexed<Q: CommandQueue>(&mut self, queue: &mut Q,
                                         attributes: SD::Attributes, indices: IndexBufferRefImpl, uniforms: SD::Uniforms,
                                         primitive: Primitive, vertex_start: usize, vertex_count: usize)
    {
        self.platform.draw_indexed::<SD, Q>(queue, attributes, indices, uniforms, primitive, vertex_start, vertex_count);
    }
}




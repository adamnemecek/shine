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


/// Trait to visit shader parameters. Mainly used for binding and uploading parameters.
#[allow(missing_docs)]
pub trait ShaderParameterVisitor<E: Engine> {
    fn process_f32x16(&mut self, idx: usize, data: &Float32x16);
    fn process_f32x4(&mut self, idx: usize, data: &Float32x4);
    fn process_f32x3(&mut self, idx: usize, data: &Float32x3);
    fn process_f32x2(&mut self, idx: usize, data: &Float32x2);
    fn process_f32(&mut self, idx: usize, data: f32);

    //fn process_tex_2d(&mut self, idx: usize, data: &R::Texture2DRef);
    //fn process_attribute(&mut self, idx: usize, data: &R::VertexAttributeRef);
    //fn process_index(&mut self, idx: usize, data: &R::IndexRef);
}


/// Trait to store shader parameters.
/// It stores both the attributes and other shader parameters.
pub trait ShaderParameters<E>: Clone {
    /// Returns the number of attributes
    fn get_count() -> usize;

    /// Returns the index by attribute name
    fn get_index_by_name(name: &str) -> Option<usize>;

    /// Bind all the required attributes for the engine
    fn bind(&self/*, visitor: &mut V*/);
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
    fn compile(&self, queue: &mut E::FrameCompose);

    /// Resets self to a new handle and compiles the shader.
    /// If handle pointed to an existing resource prior this call, that resource is not modified, Backend will
    /// garbage collect it depending on the reference count.
    fn create_and_compile(&mut self, queue: &mut E::FrameCompose) {
        self.create(queue);
        self.compile(queue);
    }

    /// Sends a geometry for rendering
    fn draw(&self, queue: &mut E::FrameCompose, parameters: DECL::Parameters,
            primitive: Primitive, vertex_start: usize, vertex_count: usize);
}

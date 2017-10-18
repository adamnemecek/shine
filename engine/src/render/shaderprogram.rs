#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;
use std::str::FromStr;
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

/// Trait to define shader attribute and uniform names
pub trait ShaderDeclaration: 'static {
    /// The enums used for the input attribute indexing.
    type Attribute: 'static + Copy + From<usize> + Into<usize> + FromStr;

    /// Returns an iterator over the shader sources
    fn get_sources() -> slice::Iter<'static, (ShaderType, &'static str)>;

    /// Returns an iterator over the possible attribute values
    fn get_attributes() -> slice::Iter<'static, Self::Attribute>;
}


/// Structure to store the shader abstraction.
pub struct ShaderProgram<SD: ShaderDeclaration> {
    pub ( crate ) platform: ShaderProgramImpl,
    phantom_sd: PhantomData<SD>,
}

impl<SD: ShaderDeclaration> ShaderProgram<SD> {
    /// Creates an empty shader.
    pub fn new() -> ShaderProgram<SD> {
        ShaderProgram {
            platform: ShaderProgramImpl::new(),
            phantom_sd: PhantomData
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

    /// Sends a geometry for rendering using the given parameters
    pub fn draw<'a, Q, ASF, USF>(&mut self, queue: &mut Q, attribute_source: ASF, uniform_source: USF, primitive: Primitive, vertex_start: usize, vertex_count: usize)
        where Q: CommandQueue, ASF: Fn(SD::Attribute) -> VertexAttributeImpl, USF: Fn(usize/*SD::Uniform*/)
    {
        let mut binding = VertexAttributeImplVec::new();
        for idx in SD::get_attributes() {
            binding.push(attribute_source(*idx));
        }

        //let mut buffer = [08; 10];
        //for idx in SD::get_uniforms() {
         //   uniform_source(*idx);
            //println!("{}", ud);
       // }
        self.platform.draw(queue, binding, primitive, vertex_start, vertex_count);
    }
}




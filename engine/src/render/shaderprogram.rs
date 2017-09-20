#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;

use render::*;

/// Enum defining the type of shader source
#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    /// Vertex shader
    VertexShader,
    /// Fragment (pixel) shader
    FragmentShader,
}


/// Structure to store the shader abstraction.
pub struct ShaderProgram<SA: PrimitiveEnum> {
    pub ( crate ) platform: ShaderProgramImpl,
    phantom_sa: PhantomData<SA>,
}

impl<SA: PrimitiveEnum> ShaderProgram<SA> {
    /// Creates an empty shader.
    pub fn new() -> ShaderProgram<SA> {
        ShaderProgram {
            platform: ShaderProgramImpl::new(),
            phantom_sa: PhantomData
        }
    }

    /// Creates a shader and attach the given sources.
    pub fn from_source<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) -> ShaderProgram<SA> {
        let mut sh = ShaderProgram {
            platform: ShaderProgramImpl::new(),
            phantom_sa: PhantomData
        };
        sh.set_sources::<I, Q>(queue, sources);
        sh
    }

    /// Releases the hw resources of a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }

    /// Attaches the sources to a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) {
        self.platform.set_sources::<SA, I, Q>(queue, sources);
    }

    /// Sends a geometry for rendering using the given parameters
    pub fn draw<'a, Q: CommandQueue, AF: Fn(SA)>(&mut self, _queue: &mut Q, _attribute_source: AF, _primitive: Primitive, _vertex_start: usize, _vertex_count: usize) {
        //self.platform.draw::<SA, I, Q>(queue, attribute_source);
    }
}



#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::fmt;
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


/// Trait to convert the shader attribute names into indices
pub trait ShaderAttributeEnum: 'static + Copy + Clone + fmt::Debug {
    /// Converts an Attribute enmu into a plain index
    fn to_index(&self) -> usize;

    /// Returns the number of attributes.
    fn count() -> usize;

    /// Converts a plain index into Attribute enum
    fn from_index(index: usize) -> Option<Self> where Self: Sized;

    /// Converts a plain index into Attribute enum
    fn from_name(name: &str) -> Option<Self> where Self: Sized;
}


/// Structure to store the shader abstraction.
pub struct ShaderProgram<SA: ShaderAttributeEnum> {
    pub ( crate ) platform: ShaderProgramImpl,
    phantom_sa: PhantomData<SA>,
}

impl<SA: ShaderAttributeEnum> ShaderProgram<SA> {
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

    /// Attaches the sources to a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) {
        self.platform.set_sources::<SA, I, Q>(queue, sources);
    }

    /// Releases the hw resources of a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }


    /// Sends a geometry for rendering using the given parameters
    pub fn draw<'a, Q: CommandQueue, AF: Fn(SA)>(&mut self, queue: &mut Q, attribute_source: AF) {
        //self.platform.draw::<SA, I, Q>(queue, attribute_source);
    }
}



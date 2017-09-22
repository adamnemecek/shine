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


/// Trait to define shader attribute and uniform names
pub trait ShaderDeclaration: 'static {
    /// The enums used for the input attribute indexing.
    type Attribute: PrimitiveEnum;
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

    /// Creates a shader and attach the given sources.
    pub fn from_source<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(queue: &mut Q, sources: I) -> ShaderProgram<SD> {
        let mut sh = ShaderProgram {
            platform: ShaderProgramImpl::new(),
            phantom_sd: PhantomData
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
        self.platform.set_sources::<SD, I, Q>(queue, sources);
    }

    /// Sends a geometry for rendering using the given parameters
    pub fn draw<'a, Q: CommandQueue, ASF: Fn(SD::Attribute) -> VertexAttributeImpl>(&mut self, queue: &mut Q, attribute_source: ASF,
                                                                                    primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        let mut binding = VertexAttributeImplVec::new();

        // init the used part
        for attribute in 0..SD::Attribute::count() {
            binding.push(attribute_source(SD::Attribute::from_index_unsafe(attribute)));
        }
        self.platform.draw(queue, binding, primitive, vertex_start, vertex_count);
    }
}




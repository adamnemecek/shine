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
    type Attribute: IterableEnum;
    /// The enums used for the input uniform indexing.
    type Uniform: IterableEnum;

    /// Iterate over the shader sources
    fn map_sources<F: FnMut((ShaderType, &str)) -> bool>(f: F) -> bool;
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
        where Q: CommandQueue, ASF: Fn(SD::Attribute) -> VertexAttributeImpl, USF: Fn(&mut SD::Uniform)
    {
        let mut binding = VertexAttributeImplVec::new();
        //let mut uniform_buffer: Vec<u8> = vec!();

        // init the used part
        for attribute_id in 0..SD::Attribute::count() {
            let attribute = SD::Attribute::from_index_unsafe(attribute_id);
            binding.push(attribute_source(attribute));
        }

        for uniform_id in 0..SD::Uniform::count() {
            let mut uniform = SD::Uniform::from_index_unsafe(uniform_id);
            uniform_source(&mut uniform);
            println!("{:?}", uniform);
            //uniform_buffer.push();
        }
        self.platform.draw(queue, binding, primitive, vertex_start, vertex_count);
    }
}




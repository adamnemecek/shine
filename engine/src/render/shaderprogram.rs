#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

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
pub struct ShaderProgram {
    pub ( crate ) platform: ShaderProgramImpl
}

impl ShaderProgram {
    /// Creates an empty shader.
    pub fn new() -> ShaderProgram {
        ShaderProgram { platform: ShaderProgramImpl::new() }
    }

    /// Creates a shader and attach the given sources.
    pub fn from_source<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) -> ShaderProgram {
        let mut sh = ShaderProgram { platform: ShaderProgramImpl::new() };
        sh.set_sources(queue, sources);
        sh
    }

    /// Attaches the sources to a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>, Q: CommandQueue>(&mut self, queue: &mut Q, sources: I) {
        self.platform.set_sources(queue, sources);
    }

    /// Releases the hw resources of a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }
}

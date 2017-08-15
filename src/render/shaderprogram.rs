use render::*;

#[derive(Copy, Clone)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}


pub struct ShaderProgram {
    pub platform: ShaderProgramImpl
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram { platform: ShaderProgramImpl::new() }
    }

    pub fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>>(&mut self, queue: &mut CommandQueue, sources: I) {
        self.platform.set_sources(queue, sources);
    }

    pub fn release(&mut self, queue: &mut CommandQueue) {
        self.platform.release(queue);
    }
}

use render::*;

#[derive(Copy,Clone)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

pub trait IShaderProgram {
    fn set_sources<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>>(&mut self, queue: &mut CommandQueue, sources: I);
    fn release(&mut self, queue: &mut CommandQueue);
}

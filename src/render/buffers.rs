use render::*;

pub trait VertexDescription {

}

pub trait IVertexBuffer {
    //fn set<'a, I: Iterator<Item=&'a (ShaderType, &'a str)>>(&mut self, queue: &mut CommandQueue, sources: I);
    fn release(&mut self, queue: &mut CommandQueue);
}

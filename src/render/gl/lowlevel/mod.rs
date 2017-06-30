pub struct LowLevel {
    //    vertexBinding: VertexBinding;
}

impl LowLevel {
    pub fn new() -> LowLevel {
        LowLevel {}
    }
}

impl Drop for LowLevel {
    fn drop(&mut self) {
        println!("drop LowLevel");
    }
}

pub struct LowLevel {
    //    vertexBinding: VertexBinding;
}

impl LowLevel {
    pub fn new() -> LowLevel {
        LowLevel {}
    }

    pub fn close(&mut self) {
        println!("close LowLevel");
        use std::{thread, time};
        thread::sleep(time::Duration::from_secs(3));
        println!("close LowLevel done");
    }
}

impl Drop for LowLevel {
    fn drop(&mut self) {
        println!("drop LowLevel");
    }
}

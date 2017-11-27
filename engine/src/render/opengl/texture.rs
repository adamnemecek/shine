#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use render::*;
use render::opengl::lowlevel::*;
//use render::opengl::lowlevel::texturebinding::*;

/// Structure to store reference to a single attribute os a buffer
#[derive(Clone)]
pub struct GLTextureRef {
    target: Rc<RefCell<GLTextureData>>
}

/*impl GLTextureRef {
    pub fn bind(&self, ll: &mut LowLevel) {
        let ib = self.target.borrow();
        ib.bind(ll);
    }
}*/


/// Structure to store hardware data associated to a IndexBuffer.
struct GLTextureData {
    hw_id: GLuint,
    type_id: GLenum,
}

impl GLTextureData {
    fn new() -> GLTextureData {
        GLTextureData {
            hw_id: 0,
            type_id: 0,
        }
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        self.hw_id = 0;
        self.type_id = 0;
    }
}

impl Drop for GLTextureData {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking texture");
    }
}


/// RenderCommand to create and allocated OpenGL resources.
struct CreateCommand {
    target: Rc<RefCell<GLTextureData>>,
}

impl Command for CreateCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {}
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Rc<RefCell<GLTextureData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}

/// IndexBuffer implementation for OpenGL.
pub struct GLTexture(Rc<RefCell<GLTextureData>>);

impl GLTexture {
    pub fn new() -> GLTexture {
        GLTexture(
            Rc::new(RefCell::new(GLTextureData::new()))
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        //println!("GLTexture - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<Q: CommandQueue>(&mut self, queue: &mut Q, width: usize, height: usize, format: PixelFormat, data: &[u8]) {
        println!("GLTexture - set_transient {},{},{:?}", width, height, format);
        /*queue.add(
            CreateCommand {
                target: self.0.clone(),
                type_id: ID::IndexType::get_gl_type_id(),
                data: index_data.to_vec(),
            }
        );*/
    }

    pub fn get_ref(&self) -> GLTextureRef {
        GLTextureRef {
            target: self.0.clone()
        }
    }
}

/// The texture implementation
pub type Texture2DImpl = GLTexture;

/// Reference to a texture
pub type Texture2DRefImpl = GLTextureRef;
mod command;
mod indexbuffer;

use core::*;
use lowlevel::*;
pub use self::command::*;
pub use self::indexbuffer::{IndexBufferIndex, IndexBufferHandle};

pub struct GLResources {
    ll: LowLevel,
}

impl GLResources {
    pub fn new() -> GLResources {
        GLResources {
            ll: LowLevel::new(),
        }
    }

    pub fn get_screen_size(&self) -> Size {
        self.ll.get_screen_size()
    }

    pub fn set_screen_size(&mut self, size: Size) {
        self.ll.set_screen_size(size);
    }

    pub fn start_render(&mut self) -> Result<(), Error> {
        self.ll.start_render();
        Ok(())
    }

    pub fn end_render(&mut self) -> Result<(), Error> {
        self.ll.end_render();
        Ok(())
    }

    pub fn ll_mut(&mut self) -> &mut LowLevel {
        &mut self.ll
    }
}

impl Resources for GLResources {
    type Command = GLCommand;

    type IndexRef = IndexBufferIndex;
    type VertexAttributeRef = u32;
    /*type Texture2D: Texture2D;
    type RenderTarget: RenderTarget;
    type ShaderProgram: ShaderProgramBase;*/
}

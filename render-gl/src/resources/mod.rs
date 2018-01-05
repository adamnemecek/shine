mod command;
mod indexbuffer;

use core::*;
pub use self::command::*;
pub use self::indexbuffer::{IndexBufferIndex, IndexBufferHandle};

pub struct GLResources {}

impl Resources for GLResources {
    type Command = GLCommand;

    type IndexRef = IndexBufferIndex;
    type VertexAttributeRef = u32;
    /*type Texture2D: Texture2D;
    type RenderTarget: RenderTarget;
    type ShaderProgram: ShaderProgramBase;*/
}

mod resource;
mod vertexbuffer;
mod indexbuffer;
mod texture2d;
mod rendertarget;
mod rendertargetconfig;
mod shaderprogram;

//mod commandstore;
//mod passmanager;
//mod rendermanager;

pub use self::resource::*;
pub use self::vertexbuffer::*;
pub use self::indexbuffer::*;
pub use self::texture2d::*;
pub use self::rendertarget::*;
pub use self::rendertargetconfig::*;
pub use self::shaderprogram::*;
//
//pub use self::commandstore::*;
//pub use self::passmanager::*;
//pub use self::rendermanager::*;

pub trait Resources: 'static {
    type Command: Command;
}

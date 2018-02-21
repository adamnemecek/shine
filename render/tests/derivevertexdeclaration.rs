extern crate shine_render as render;

use render::*;

#[derive(Copy, Clone, Debug)]
#[derive(VertexDeclaration)]
struct VxPos {
    position: Float32x3,
}

extern crate gl;


struct VertexAttribute
{
    type: gl::GLenum,
    components : gl::GLuint,
    normalize: gl::GLboolean,
    stride: gl::GLsizei,
    offset: gl::GLintptr
}
/*
private struct BoundVertexAttribute
{
    ubyte timeStamp;                        ///< Helper to disable unbound attributes b/n consecutive render calls
    VertexAttribute current;                ///< Current binding info
}


struct VertexBinding {
    boundVertexBufferId
}*/
use arrayvec::ArrayVec;
use std::marker::PhantomData;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::lowlevel::vertexbinding::*;
use store::handlestore::*;


type GLVertexBufferAttributeVec = ArrayVec<[GLVertexBufferAttribute; MAX_VERTEX_ATTRIBUTE_COUNT]>;


/// Structure to store hardware data associated to a VertexBuffer.
pub struct GLVertexBuffer {
    hw_id: GLuint,
    attributes: GLVertexBufferAttributeVec,
}

impl GLVertexBuffer {
    pub fn new() -> GLVertexBuffer {
        GLVertexBuffer {
            hw_id: 0,
            attributes: GLVertexBufferAttributeVec::new(),
        }
    }

    pub fn upload_data<VD: VertexDeclaration>(&mut self, ll: &mut LowLevel, data: &[u8]) {
        for idx in VD::get_attributes() {
            self.attributes.push(VD::get_attribute_layout(*idx));
            assert!(self.attributes.len() <= MAX_VERTEX_ATTRIBUTE_COUNT, "Vertex attribute count exceeds engine limits ({})", MAX_VERTEX_ATTRIBUTE_COUNT);
        }

        gl_check_error();
        if self.hw_id == 0 {
            gl!(GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);

        ll.vertex_binding.bind_buffer(self.hw_id);
        gl!(BufferData(gl::ARRAY_BUFFER,
                       data.len() as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW));

        gl_check_error();
    }

    pub fn bind(&self, ll: &mut LowLevel, location: GLuint, attribute: usize) {
        ll.vertex_binding.bind_attribute(location, self.hw_id, &self.attributes[attribute as usize]);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.vertex_binding.unbind_if_active(self.hw_id);
        gl!(DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
    }
}

impl Drop for GLVertexBuffer {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking vertex buffer");
    }
}


/// VertexBuffer implementation for OpenGL.
impl<DECL: VertexDeclaration> VertexBuffer<DECL> for VertexBufferHandle<DECL> {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        struct ReleaseCommand {
            target: Index<GLVertexBuffer>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                0
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources.vertex_buffers[&self.target];
                target.release(ll);
            }
        }

        //println!("GLVertexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    fn set<'a, SRC: VertexSource<DECL>, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC) {
        /// RenderCommand to create and allocated OpenGL resources.
        struct CreateCommand<VD: VertexDeclaration> {
            target: Index<GLVertexBuffer>,
            data: Vec<u8>,
            phantom_data: PhantomData<VD>,
        }

        impl<VD: VertexDeclaration> Command for CreateCommand<VD> {
            fn get_sort_key(&self) -> usize {
                0
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources.vertex_buffers[&self.target];
                target.upload_data::<VD>(ll, self.data.as_slice());
            }
        }

        match source.to_data() {
            VertexData::Transient(slice) => {
                //println!("GLVertexBuffer - set_copy");
                assert!(!self.is_null());

                queue.add(
                    CreateCommand::<DECL> {
                        target: self.0.clone(),
                        data: slice.to_vec(),
                        phantom_data: PhantomData,
                    }
                );
            }
        }
    }
}


/// Describe the memory layout of a vertex attribute
pub type VertexBufferLayoutElementImpl = GLVertexBufferAttribute;

/// The vertex buffer implementation
pub type VertexBufferImpl = GLVertexBuffer;

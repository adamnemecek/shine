use backend::*;
use backend::opengl::lowlevel::*;
use store::handlestore::*;


/// Structure to store hardware data associated to a IndexBuffer.
pub struct GLIndexBuffer {
    hw_id: GLuint,
    type_id: GLenum,
}

impl GLIndexBuffer {
    pub fn new() -> GLIndexBuffer {
        GLIndexBuffer {
            hw_id: 0,
            type_id: 0,
        }
    }

    pub fn upload_data(&mut self, ll: &mut LowLevel, type_id: GLenum, data: &[u8]) {
        gl_check_error();
        if self.hw_id == 0 {
            gl!(GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);
        self.type_id = type_id;

        ll.index_binding.bind_buffer(self.hw_id);
        gl!(BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       data.len() as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW));
        gl_check_error();
    }

    pub fn bind(&self, ll: &mut LowLevel) {
        ll.index_binding.bind_index(self.hw_id, self.type_id);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.index_binding.unbind_if_active(self.hw_id);
        gl!(DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
        self.type_id = 0;
    }
}

impl Drop for GLIndexBuffer {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking index buffer");
    }
}


impl<DECL: IndexDeclaration> Resource for IndexBufferHandle<DECL> {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        struct ReleaseCommand {
            target: UnsafeIndex<GLIndexBuffer>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                0
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.release(ll);
            }
        }


        //println!("GLIndexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: UnsafeIndex::from_index(&self.0),
            }
        );
    }
}

impl<DECL: IndexDeclaration> IndexBuffer<DECL> for IndexBufferHandle<DECL> {
    fn set<'a, SRC: IndexSource<DECL>, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC) {
        /// RenderCommand to create and allocated OpenGL resources.
        struct CreateCommand {
            target: UnsafeIndex<GLIndexBuffer>,
            type_id: GLenum,
            data: Vec<u8>,
        }

        impl Command for CreateCommand {
            fn get_sort_key(&self) -> usize {
                0
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.upload_data(ll, self.type_id, self.data.as_slice());
            }
        }

        match source.to_data() {
            IndexData::Transient(slice) => {
                //println!("GLIndexBuffer - set_copy");
                assert!(!self.is_null());

                queue.add(
                    CreateCommand {
                        target: UnsafeIndex::from_index(&self.0),
                        type_id: From::from(DECL::get_layout()),
                        data: slice.to_vec(),
                    }
                );
            }
        }
    }
}


/*use std::marker::PhantomData;
use backend::*;

use store::handlestore::*;
use backend::indexbuffer::IndexBufferImpl;

crate type IndexBufferStore = Store<IndexBufferImpl>;
crate type GuardedIndexBufferStore<'a> = UpdateGuardStore<'a, IndexBufferImpl>;
crate type IndexBufferIndex = Index<IndexBufferImpl>;
pub type UnsafeIndexBufferIndex = UnsafeIndex<IndexBufferImpl>;


/// Handle to an index buffer resource
#[derive(Clone)]
pub struct IndexBufferHandle<DECL: IndexDeclaration>( crate IndexBufferIndex, PhantomData<DECL>);

impl<DECL: IndexDeclaration> IndexBufferHandle<DECL> {
    pub fn null() -> IndexBufferHandle<DECL> {
        IndexBufferHandle(IndexBufferIndex::null(), PhantomData)
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> IndexBufferHandle<DECL> {
        IndexBufferHandle(res.resources.index_buffers.add(IndexBufferImpl::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}

impl<'a, DECL: IndexDeclaration> From<&'a IndexBufferHandle<DECL>> for UnsafeIndex<IndexBufferImpl> {
    #[inline(always)]
    fn from(idx: &IndexBufferHandle<DECL>) -> UnsafeIndex<IndexBufferImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}

pub struct NoIndex;

impl From<NoIndex> for UnsafeIndex<IndexBufferImpl> {
    #[inline(always)]
    fn from(_idx: NoIndex) -> UnsafeIndex<IndexBufferImpl> {
        UnsafeIndex::null()
    }
}

*/

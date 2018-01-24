use arrayvec::ArrayVec;
use std::marker::PhantomData;

use engine::*;
use opengl::*;
use opengl::lowlevel::vertexbinding::*;
use store::store::*;


pub type GLVertexBufferStore = Store<GLVertexBuffer>;



/*

pub type VertexBufferStore = Store<GLVertexBuffer>;
pub type GuardedVertexBufferStore<'a> = UpdateGuardStore<'a, GLVertexBuffer>;
pub type VertexBufferIndex = Index<GLVertexBuffer>;

/// Handle to a vertex buffer resource
#[derive(Clone)]
pub struct VertexBufferHandle<DECL: VertexDeclaration>(VertexBufferIndex, PhantomData<DECL>);

impl<DECL: VertexDeclaration> VertexBufferHandle<DECL> {
    pub fn null() -> VertexBufferHandle<DECL> {
        VertexBufferHandle(VertexBufferIndex::null(), PhantomData)
    }

    /*pub fn create<K: PassKey>(res: &mut RenderManager<K,>) -> VertexBufferHandle<DECL> {
        VertexBufferHandle(res.resources.vertex_buffers.add(VertexBufferImpl::new()), PhantomData)
    }*/

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}

/// VertexBuffer implementation for OpenGL.
impl<DECL: VertexDeclaration> Resource for VertexBufferHandle<DECL> {
    fn release<Q: CommandQueue>(&self, queue: &Q) {
        struct ReleaseCommand {
            target: UnsafeIndex<GLVertexBuffer>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                0
            }
        }

        impl GLCommand for ReleaseCommand {
            /*fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.release(ll);
            }*/
        }

        //println!("GLVertexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: UnsafeIndex::from_index(&self.0),
            }
        );
    }
}

impl<DECL: VertexDeclaration> VertexBufferBase for VertexBufferHandle<DECL> {
    type AttributeRef = (VertexBufferIndex, usize);
}

impl<DECL: VertexDeclaration> VertexBuffer<DECL> for VertexBufferHandle<DECL> {
    fn set<'a, SRC: VertexSource<DECL>, Q: CommandQueue>(&self, queue: &Q, source: &SRC) {
        /// RenderCommand to create and allocated OpenGL resources.
        struct CreateCommand<VD: VertexDeclaration> {
            target: UnsafeIndex<GLVertexBuffer>,
            data: Vec<u8>,
            phantom_data: PhantomData<VD>,
        }

        impl<VD: VertexDeclaration> Command for CreateCommand<VD> {
            fn get_sort_key(&self) -> usize {
                0
            }
        }

        impl<VD: VertexDeclaration> GLCommand for CreateCommand<VD> {
            /*fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.upload_data::<VD>(ll, self.data.as_slice());
            }*/
        }

        match source.to_data() {
            VertexData::Transient(slice) => {
                //println!("GLVertexBuffer - set_copy");
                assert!(!self.is_null());

                queue.add(
                    CreateCommand::<DECL> {
                        target: UnsafeIndex::from_index(&self.0),
                        data: slice.to_vec(),
                        phantom_data: PhantomData,
                    }
                );
            }
        }
    }

    fn get_attribute(&self, attr: DECL::Attribute) -> (VertexBufferIndex, usize) {
        (self.0.clone(), attr.into())
    }
}
*/
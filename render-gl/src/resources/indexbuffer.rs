#![allow(dead_code)]

use std::marker::PhantomData;
use core::*;
use lowlevel::*;
use resources::*;
use store::store::*;

/// Command to release and index buffer
pub struct ReleaseCommand {
    target: UnsafeIndex<GLIndexBuffer>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }
}

/*impl GLCommand for ReleaseCommand {
    /*fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        let target = &mut resources[&self.target];
        target.release(ll);
    }*/
}*/

impl From<ReleaseCommand> for GLCommand {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> GLCommand {
        GLCommand::IndexRelease(value)
    }
}


/// Command to create an index buffer
pub struct CreateCommand {
    target: UnsafeIndex<GLIndexBuffer>,
    type_id: GLenum,
    data: Vec<u8>,
}

impl From<CreateCommand> for GLCommand {
    #[inline(always)]
    fn from(value: CreateCommand) -> GLCommand {
        GLCommand::IndexCreate(value)
    }
}


pub type IndexBufferStore = Store<GLIndexBuffer>;
pub type GuardedIndexBuffer<'a> = UpdateGuard<'a, GLIndexBuffer>;
pub type IndexBufferIndex = Index<GLIndexBuffer>;

/// Handle to an index buffer resource
#[derive(Clone)]
pub struct IndexBufferHandle<DECL: IndexDeclaration>(IndexBufferIndex, PhantomData<DECL>);

impl<DECL: IndexDeclaration> IndexBufferHandle<DECL> {
    pub fn null() -> IndexBufferHandle<DECL> {
        IndexBufferHandle(IndexBufferIndex::null(), PhantomData)
    }

    /*pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> IndexBufferHandle<DECL> {
        IndexBufferHandle(res.resources.index_buffers.add(IndexBufferImpl::new()), PhantomData)
    }*/

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}

impl<DECL: IndexDeclaration> Resource for IndexBufferHandle<DECL> {
    type Command = GLCommand;

    fn release<Q: CommandQueue<Command=Self::Command>>(&self, queue: &Q) {
        println!("GLIndexBuffer - release");
        queue.add(
            ReleaseCommand {
                target: UnsafeIndex::from_index(&self.0),
            }.into()
        );
    }
}

impl<DECL: IndexDeclaration> IndexBuffer<DECL> for IndexBufferHandle<DECL> {
    fn set<'a, SRC: IndexSource<DECL>, Q: CommandQueue<Command=Self::Command>>(&self, queue: &Q, source: &SRC) {
        match source.to_data() {
            IndexData::Transient(slice) => {
                println!("GLIndexBuffer - set_copy");
                assert!(!self.is_null());
                queue.add(
                    CreateCommand {
                        target: UnsafeIndex::from_index(&self.0),
                        type_id: IndexBinding::glenum_from_index_type(DECL::get_layout()),
                        data: slice.to_vec(),
                    }.into()
                );
            }
        }
    }
}

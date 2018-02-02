use std::marker::PhantomData;
use core::*;
use lowlevel::*;
use framework::*;
use resources::*;
use store::store::*;


/// Command to create or update index buffer
pub struct CreateCommand {
    target: UnsafeIndex<GLIndexBuffer>,
    type_id: GLenum,
    data: Vec<u8>,
}

impl CreateCommand {
    pub fn process(self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.index_store.at_unsafe_mut(&self.target) };
        target.upload_data(context.ll, self.type_id, &self.data);
    }
}

impl From<CreateCommand> for Command {
    #[inline(always)]
    fn from(value: CreateCommand) -> Command {
        Command::IndexCreate(value)
    }
}


/// Command to release an index buffer
pub struct ReleaseCommand {
    target: UnsafeIndex<GLIndexBuffer>,
}

impl ReleaseCommand {
    pub fn process(self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.index_store.at_unsafe_mut(&self.target) };
        target.release(context.ll);
    }
}

impl From<ReleaseCommand> for Command {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> Command {
        Command::IndexRelease(value)
    }
}


pub type IndexBufferStore = Store<GLIndexBuffer>;
pub type ReadGuardIndexBuffer<'a> = ReadGuard<'a, GLIndexBuffer>;
pub type WriteGuardIndexBuffer<'a> = WriteGuard<'a, GLIndexBuffer>;
pub type IndexBufferIndex = Index<GLIndexBuffer>;
pub type UnsafeIndexBufferIndex = UnsafeIndex<GLIndexBuffer>;


/// Handle to an index buffer
#[derive(Clone)]
pub struct IndexBufferHandle<DECL: IndexDeclaration>(IndexBufferIndex, PhantomData<DECL>);

impl<DECL: IndexDeclaration> Handle for IndexBufferHandle<DECL> {
    fn null() -> IndexBufferHandle<DECL> {
        IndexBufferHandle(IndexBufferIndex::null(), PhantomData)
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<DECL: IndexDeclaration> Resource<PlatformEngine> for IndexBufferHandle<DECL> {
    fn create(&mut self, compose: &mut GLCommandQueue) {
        self.0 = compose.add_index_buffer(GLIndexBuffer::new());
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn release(&self, queue: &mut GLCommandQueue) {
        if self.is_null() {
            return;
        }

        println!("IndexBuffer - release");
        queue.add_command(0,
                          ReleaseCommand {
                              target: UnsafeIndex::from_index(&self.0),
                          });
    }
}

impl<DECL: IndexDeclaration> IndexBuffer<DECL, PlatformEngine> for IndexBufferHandle<DECL> {
    fn set<SRC: IndexSource<DECL>>(&self, queue: &mut GLCommandQueue, source: &SRC) {
        assert!(!self.is_null());

        match source.to_data() {
            IndexData::Transient(slice) => {
                println!("IndexBuffer - IndexData::Transient");
                queue.add_command(0,
                                  CreateCommand {
                                      target: UnsafeIndex::from_index(&self.0),
                                      type_id: IndexBinding::glenum_from_index_type(DECL::get_layout()),
                                      data: slice.to_vec(),
                                  });
            }
        }
    }
}

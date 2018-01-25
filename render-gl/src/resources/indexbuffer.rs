#![allow(dead_code)]

//use std::marker::PhantomData;
//use core::*;
use lowlevel::*;
use resources::*;
use store::store::*;


/// Command to release and index buffer
pub struct ReleaseCommand {
    target: UnsafeIndex<GLIndexBuffer>,
}

impl ReleaseCommand {
    pub fn process<'a>(&mut self, _ll: &mut LowLevel, _flush: &mut GLFrameFlush) {
        //let target = &mut resources[&self.target];
        //target.release(ll);
    }
}

impl From<ReleaseCommand> for Command {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> Command {
        Command::IndexRelease(value)
    }
}


/// Command to create an index buffer
pub struct CreateCommand {
    target: UnsafeIndex<GLIndexBuffer>,
    type_id: GLenum,
    data: Vec<u8>,
}

impl CreateCommand {
    pub fn process<'a>(&mut self, _ll: &mut LowLevel, _flush: &mut GLFrameFlush) {
        //let target = &mut resources[&self.target];
        //target.release(ll);
    }
}

impl From<CreateCommand> for Command {
    #[inline(always)]
    fn from(value: CreateCommand) -> Command {
        Command::IndexCreate(value)
    }
}


pub type IndexBufferStore = Store<GLIndexBuffer>;
pub type ReadGuardIndexBuffer<'a> = ReadGuard<'a, GLIndexBuffer>;
pub type WriteGuardIndexBuffer<'a> = WriteGuard<'a, GLIndexBuffer>;
pub type IndexBufferIndex = Index<GLIndexBuffer>;

/// Handle to an index buffer resource
#[derive(Clone)]
pub struct IndexBufferHandle<DECL: IndexDeclaration>(IndexBufferIndex, PhantomData<DECL>);

impl<DECL: IndexDeclaration> IndexBufferHandle<DECL> {
    pub fn null() -> IndexBufferHandle<DECL> {
        IndexBufferHandle(IndexBufferIndex::null(), PhantomData)
    }

    pub fn create(compose: &mut GLFrameCompose) -> IndexBufferHandle<DECL> {
        IndexBufferHandle(compose.index_store.add(GLIndexBuffer::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}

impl<DECL: IndexDeclaration> Resource<PlatformEngine> for IndexBufferHandle<DECL> {
    fn release(&self, queue: &mut GLFrameCompose) {
        println!("GLIndexBuffer - release");
        queue.command_queue.add( 
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
* /
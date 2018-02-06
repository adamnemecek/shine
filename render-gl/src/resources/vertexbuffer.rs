use std::marker::PhantomData;
use core::*;
use lowlevel::*;
use framework::*;
use resources::*;
use store::store::*;


/// Command to create or update vertex buffer
pub struct CreateCommand<DECL: VertexDeclaration> {
    target: UnsafeIndex<GLVertexBuffer>,
    data: Vec<u8>,
    phantom: PhantomData<DECL>,
}

impl<DECL: VertexDeclaration> DynCommand for CreateCommand<DECL> {
    fn process(&mut self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.vertex_store.at_unsafe_mut(&self.target) };
        let layout = DECL::attribute_layout_iter().map(|a| GLVertexBufferAttribute::from_layout(&a));
        target.upload_data(context.ll, layout, &self.data);
    }
}


/// Command to release a vertex buffer
pub struct ReleaseCommand {
    target: UnsafeIndex<GLVertexBuffer>,
}

impl ReleaseCommand {
    pub fn process(self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.vertex_store.at_unsafe_mut(&self.target) };
        target.release(context.ll);
    }
}

impl From<ReleaseCommand> for Command {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> Command {
        Command::VertexRelease(value)
    }
}


pub type VertexBufferStore = Store<GLVertexBuffer>;
pub type ReadGuardVertexBuffer<'a> = ReadGuard<'a, GLVertexBuffer>;
pub type WriteGuardVertexBuffer<'a> = WriteGuard<'a, GLVertexBuffer>;
pub type VertexBufferIndex = Index<GLVertexBuffer>;
pub type UnsafeVertexBufferIndex = UnsafeIndex<GLVertexBuffer>;


/// Handle to a vertex buffer
#[derive(Clone)]
pub struct VertexBufferHandle<DECL: VertexDeclaration>(VertexBufferIndex, PhantomData<DECL>);

impl<DECL: VertexDeclaration> Handle for VertexBufferHandle<DECL> {
    fn null() -> VertexBufferHandle<DECL> {
        VertexBufferHandle(VertexBufferIndex::null(), PhantomData)
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

/// VertexBuffer implementation for OpenGL.
impl<DECL: VertexDeclaration> Resource<PlatformEngine> for VertexBufferHandle<DECL> {
    fn create(&mut self, compose: &mut GLCommandQueue) {
        self.0 = compose.add_vertex_buffer(GLVertexBuffer::new());
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn release(&self, queue: &mut GLCommandQueue) {
        if self.is_null() {
            return;
        }

        //println!("VertexBuffer - release");
        queue.add_command(0,
                          ReleaseCommand {
                              target: UnsafeIndex::from_index(&self.0),
                          });
    }
}

impl<DECL: VertexDeclaration> VertexBuffer<DECL, PlatformEngine> for VertexBufferHandle<DECL> {
    fn set<'a, SRC: VertexSource<DECL>>(&self, queue: &mut GLCommandQueue, source: &SRC) {
        assert!(!self.is_null());

        match source.to_data() {
            VertexData::Transient(slice) => {
                //println!("VertexBuffer - VertexData::Transient");
                queue.add_command(0,
                                  CreateCommand::<DECL> {
                                      target: UnsafeIndex::from_index(&self.0),
                                      data: slice.to_vec(),
                                      phantom: PhantomData,
                                  });
            }
        }
    }
}


/// Vertex attribute reference.
#[derive(Clone, Debug)]
pub struct UnsafeVertexAttributeIndex(pub UnsafeVertexBufferIndex, pub usize);

impl<'a, DECL: VertexDeclaration> From<(&'a VertexBufferHandle<DECL>, DECL::Attribute)> for UnsafeVertexAttributeIndex {
    #[inline(always)]
    fn from(value: (&VertexBufferHandle<DECL>, DECL::Attribute)) -> UnsafeVertexAttributeIndex {
        UnsafeVertexAttributeIndex(UnsafeVertexBufferIndex::from_index(&(value.0).0), value.1.into())
    }
}

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::mem;
use std::slice;
use std::marker::PhantomData;

use render::*;


/// Trait to define vertex declaration.
pub trait VertexDeclaration: 'static {
    /// The enums used for the attribute indexing.
    type Attribute: PrimitiveEnum;

    /// Returns the platform dependent vertex attribute description.
    fn get_attribute_descriptor(index: usize) -> VertexAttributeDescriptorImpl;
}


/// Trait to define vertex declaration.
pub trait TransientVertexSource<VD: VertexDeclaration> {
    /// Returns the vertex declaration and the reference to the vertex data.
    fn to_vertex_data<'a>(&self) -> &'a [u8];
}


/// TransientVertexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, slice can be used:
///
/// let data = [Vertex; 1024];
/// let desc = data.as_ref().to_vertex_data();
///
macro_rules! __impl_array_TransientVertexSource {
    ($($N:expr)+) => {
        $(
            /// TransientVertexSource implementation for array.
            impl<VD: VertexDeclaration + Sized> TransientVertexSource<VD> for [VD;$N] {
                fn to_vertex_data<'a>(&self) -> &'a [u8]
                {
                    unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<VD>()) }
                }
            }
        )+
    }
}

__impl_array_TransientVertexSource! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}


/// TransientVertexSource implementation for slice.
impl<'a, VD: 'a + VertexDeclaration + Sized> TransientVertexSource<VD> for &'a [VD] {
    fn to_vertex_data<'b>(&self) -> &'b [u8]
    {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<VD>()) }
    }
}


/// TransientVertexSource implementation for Vec.
impl<VD: VertexDeclaration + Sized> TransientVertexSource<VD> for Vec<VD> {
    fn to_vertex_data<'a>(&self) -> &'a [u8]
    {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<VD>()) }
    }
}


/// Structure to store a vertex buffer
pub struct VertexBuffer<VD: VertexDeclaration> {
    pub ( crate ) platform: VertexBufferImpl,
    phantom_vd: PhantomData<VD>,
}

impl<VD: VertexDeclaration> VertexBuffer<VD> {
    /// Creates an empty shader.
    pub fn new() -> VertexBuffer<VD> {
        VertexBuffer {
            platform: VertexBufferImpl::new(),
            phantom_vd: PhantomData,
        }
    }

    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }

    /// Sets the content of the buffer from a transient source.
    ///
    /// Transient means that, the source my be modified, droped after the function call, thus
    /// a copy is created from the data.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_transient<'a, VS: TransientVertexSource<VD>, Q: CommandQueue>(&mut self, queue: &mut Q, vertex_source: &VS) {
        assert!(VD::Attribute::count() <= MAX_VERTEX_ATTRIBUTE_COUNT, "vertex attribute count exceeds engine limits ({})", MAX_VERTEX_ATTRIBUTE_COUNT);
        let mut attributes = VertexAttributeDescriptorImplVec::new();
        for idx in 0..VD::Attribute::count() {
            attributes.push(VD::get_attribute_descriptor(idx));
        }
        self.platform.set_transient(queue, attributes, vertex_source.to_vertex_data());
    }

    /// Returns reference to an attribute
    pub fn get_attribute(&self, attr: VD::Attribute) -> VertexAttributeImpl {
        self.platform.get_attribute(attr.to_index())
    }
}

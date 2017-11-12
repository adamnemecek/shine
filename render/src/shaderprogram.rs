use std::slice;
use std::marker::PhantomData;
use backend::*;


/// Enum defining the type of shader source
#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    /// Vertex shader
    VertexShader,
    /// Fragment (pixel) shader
    FragmentShader,
}


/// Trait to visit shader parameters. Mainly used for binding and uploading parameters.
#[allow(missing_docs)]
pub trait ShaderParameterVisitor {
    fn process_f32x16(&mut self, idx: usize, data: &Float32x16);
    fn process_f32x4(&mut self, idx: usize, data: &Float32x4);
    fn process_f32x3(&mut self, idx: usize, data: &Float32x3);
    fn process_f32x2(&mut self, idx: usize, data: &Float32x2);
    fn process_f32(&mut self, idx: usize, data: f32);

    fn process_tex_2d(&mut self, idx: usize, data: &UnsafeTexture2DIndex);
    fn process_attribute(&mut self, idx: usize, data: &UnsafeVertexAttributeHandle);
    fn process_index(&mut self, idx: usize, data: &UnsafeIndexBufferIndex);
}


/// Trait to store shader parameters.
/// It stores both the attributes and other shader parameters.
pub trait ShaderParameters: Clone {
    /// Returns the number of attributes
    fn get_count() -> usize;

    /// Returns the index by attribute name
    fn get_index_by_name(name: &str) -> Option<usize>;

    /// Visit all the required attributes
    fn visit<V: ShaderParameterVisitor>(&self, visitor: &mut V);
}


/// Trait to define shader attribute and uniform names
pub trait ShaderDeclaration: 'static {
    /// The structure storing the shader parameters.
    type Parameters: ShaderParameters;

    /// Returns an iterator over the shader sources
    fn get_sources() -> slice::Iter<'static, (ShaderType, &'static str)>;
}


/// Structure to store the shader abstraction.
pub trait ShaderProgram<DECL: ShaderDeclaration> {
    /// Uploads and compiles the shader.
    fn compile<Q: CommandQueue>(&self, queue: &mut Q);

    /// Releases the hw resources of a shader.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);

    /// Sends a geometry for rendering
    fn draw<Q: CommandQueue>(&self, queue: &mut Q, parameters: DECL::Parameters,
                             primitive: Primitive, vertex_start: usize, vertex_count: usize);
}


use store::handlestore::*;

crate type ShaderProgramStore = Store<ShaderProgramImpl>;
crate type GuardedShaderProgramStore<'a> = UpdateGuardStore<'a, ShaderProgramImpl>;
crate type ShaderProgramIndex = Index<ShaderProgramImpl>;
pub type UnsafeShaderProgramIndex = UnsafeIndex<ShaderProgramImpl>;


/// Handle to a texture 2d resource
#[derive(Clone)]
pub struct ShaderProgramHandle<DECL: ShaderDeclaration>( crate ShaderProgramIndex, PhantomData<DECL>);

impl<DECL: ShaderDeclaration> ShaderProgramHandle<DECL> {
    pub fn null() -> ShaderProgramHandle<DECL> {
        ShaderProgramHandle(ShaderProgramIndex::null(), PhantomData)
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> ShaderProgramHandle<DECL> {
        ShaderProgramHandle(res.resources.shaders.add(ShaderProgramImpl::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn as_ref(&self) -> UnsafeIndex<ShaderProgramImpl> {
        UnsafeIndex::from_index(&self.0)
    }
}

impl<'a, DECL: ShaderDeclaration> From<&'a ShaderProgramHandle<DECL>> for UnsafeIndex<ShaderProgramImpl> {
    #[inline(always)]
    fn from(idx: &ShaderProgramHandle<DECL>) -> UnsafeIndex<ShaderProgramImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}


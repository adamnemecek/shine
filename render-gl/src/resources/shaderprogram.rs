use std::marker::PhantomData;
use core::*;
use lowlevel::*;
use framework::*;
use resources::*;
use store::store::*;


/// Command to create or update index buffer
pub struct CreateCommand<DECL: ShaderDeclaration<PlatformEngine>> {
    target: UnsafeIndex<GLShaderProgram>,
    phantom: PhantomData<DECL>,
}

impl<DECL: ShaderDeclaration<PlatformEngine>> DynCommand for CreateCommand<DECL> {
    fn process(&mut self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.shader_program_store.at_unsafe_mut(&self.target) };
        target.create_program(context.ll, DECL::source_iter());
    }
}


/// Command to release a shader
pub struct ReleaseCommand {
    target: UnsafeIndex<GLShaderProgram>,
}

impl ReleaseCommand {
    pub fn process(self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.shader_program_store.at_unsafe_mut(&self.target) };
        target.release(context.ll);
    }
}

impl From<ReleaseCommand> for Command {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> Command {
        Command::ShaderProgramRelease(value)
    }
}


/// Command to draw primitive with the given shader
struct DrawCommand<DECL: ShaderDeclaration<PlatformEngine>> {
    target: UnsafeIndex<GLShaderProgram>,
    parameters: DECL::Parameters,
    primitive: GLenum,
    vertex_start: GLuint,
    vertex_count: GLuint,
}

impl<DECL: ShaderDeclaration<PlatformEngine>> DynCommand for DrawCommand<DECL> {
    fn process(&mut self, context: &mut GLCommandProcessContext) {
        let target = unsafe { context.shader_program_store.at_unsafe_mut(&self.target) };
        //self.parameters.bind(context);
        //target.draw(resources, ll, &self.parameters, self.primitive, self.vertex_start, self.vertex_count);
    }
}


pub type ShaderProgramStore = Store<GLShaderProgram>;
pub type ReadGuardShaderProgram<'a> = ReadGuard<'a, GLShaderProgram>;
pub type WriteGuardShaderProgram<'a> = WriteGuard<'a, GLShaderProgram>;
pub type ShaderProgramIndex = Index<GLShaderProgram>;
pub type UnsafeShaderProgramIndex = UnsafeIndex<GLShaderProgram>;


/// Handle to an index buffer
#[derive(Clone)]
pub struct ShaderProgramHandle<DECL: ShaderDeclaration<PlatformEngine>>(ShaderProgramIndex, PhantomData<DECL>);

impl<DECL: ShaderDeclaration<PlatformEngine>> Handle for ShaderProgramHandle<DECL> {
    fn null() -> ShaderProgramHandle<DECL> {
        ShaderProgramHandle(ShaderProgramIndex::null(), PhantomData)
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<DECL: ShaderDeclaration<PlatformEngine>> Resource<PlatformEngine> for ShaderProgramHandle<DECL> {
    fn create(&mut self, compose: &mut GLCommandQueue) {
        self.0 = compose.add_shader_program(GLShaderProgram::new());
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn release(&self, queue: &mut GLCommandQueue) {
        if self.is_null() {
            return;
        }

        println!("ShaderProgram - release");
        queue.add_command(0,
                          ReleaseCommand {
                              target: UnsafeIndex::from_index(&self.0),
                          });
    }
}

impl<DECL: ShaderDeclaration<PlatformEngine>> ShaderProgram<DECL, PlatformEngine> for ShaderProgramHandle<DECL> {
    fn compile(&self, queue: &mut GLCommandQueue) {
        println!("ShaderProgram - compile");
        queue.add_command(0,
                          CreateCommand::<DECL> {
                              target: UnsafeIndex::from_index(&self.0),
                              phantom: PhantomData,
                          });
    }

    fn draw(&self, queue: &mut GLCommandQueue, parameters: DECL::Parameters, primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        println!("ShaderProgram - draw");
        queue.add_command(0,
                          DrawCommand::<DECL> {
                              target: UnsafeIndex::from_index(&self.0),
                              parameters: parameters,
                              primitive: glenum_from_primitive(primitive),
                              vertex_start: vertex_start as u32,
                              vertex_count: vertex_count as u32,
                          });
    }
}
















/*use std::str::from_utf8;
use std::marker::PhantomData;
use std::mem;

use arrayvec::ArrayVec;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::commandqueue::*;
use store::handlestore::*;

/// Helper to upload shader parameters
struct ParameterUploader<'a, 'r: 'a> {
    locations: &'a ParameterLocations,
    ll: &'a mut LowLevel,
    resources: &'a mut GuardedResources<'r>,
}

impl<'a, 'r> ShaderParameterVisitor for ParameterUploader<'a, 'r> {
    fn process_f32x16(&mut self, idx: usize, data: &Float32x16) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_MAT4 && size == 1);
                gl_check_error();
                gl!(UniformMatrix4fv(location as i32, size, gl::FALSE, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x4(&mut self, idx: usize, data: &Float32x4) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC4 && size == 1);
                gl_check_error();
                gl!(Uniform4fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x3(&mut self, idx: usize, data: &Float32x3) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC3 && size == 1);
                gl_check_error();
                gl!(Uniform3fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32x2(&mut self, idx: usize, data: &Float32x2) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT_VEC2 && size == 1);
                gl_check_error();
                gl!(Uniform2fv(location as i32, size, mem::transmute(data)));
                gl_check_error();
            }
        }
    }

    fn process_f32(&mut self, idx: usize, data: f32) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::FLOAT && size == 1);
                gl_check_error();
                gl!(Uniform1fv(location as i32, size, mem::transmute(&data)));
                gl_check_error();
            }
        }
    }

    fn process_tex_2d(&mut self, idx: usize, data: &UnsafeTexture2DIndex) {
        if let ParameterLocation::Uniform { location, size, type_id } = self.locations[idx] {
            if type_id != 0 {
                assert!(type_id == gl::SAMPLER_2D && size == 1);
                gl_check_error();
                let texture = &mut self.resources[data];
                let slot = texture.bind(self.ll);
                let slot = slot as u32;
                gl!(Uniform1i(location as i32, slot as i32));
                gl_check_error();
            }
        }
    }

    fn process_attribute(&mut self, idx: usize, data: &UnsafeVertexAttributeHandle) {
        if let ParameterLocation::Attribute { location, type_id, .. } = self.locations[idx] {
            if type_id != 0 {
                gl_check_error();
                let buffer = &mut self.resources[data];
                buffer.bind(self.ll, location, data.1);
                gl_check_error();
            }
        }
    }

    fn process_index(&mut self, _idx: usize, data: &UnsafeIndexBufferIndex) {
        gl_check_error();
        if data.is_null() {
            self.ll.index_binding.bind_no_index();
        } else {
            let buffer = &mut self.resources[data];
            buffer.bind(self.ll);
        }
        gl_check_error();
    }
}


/// Structure to store the shader abstraction.
impl<DECL: ShaderDeclaration> Resource for ShaderProgramHandle<DECL> {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        struct ReleaseCommand {
            target: UnsafeIndex<GLShaderProgram>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.release(ll);
            }
        }

        queue.add(
            ReleaseCommand {
                target: UnsafeIndex::from_index(&self.0),
            }
        );
    }
}

impl<DECL: ShaderDeclaration> ShaderProgram<DECL> for ShaderProgramHandle<DECL> {
    fn compile<Q: CommandQueue>(&self, queue: &mut Q) {
        /// RenderCommand to allocate the OpenGL program, set the shader sources and compile (link) a shader program
        struct CreateCommand<SD: ShaderDeclaration> {
            target: UnsafeIndex<GLShaderProgram>,
            phantom_sd: PhantomData<SD>,
        }

        impl<SD: ShaderDeclaration> Command for CreateCommand<SD> {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.create_program::<SD>(ll);
                target.parse_parameters::<SD>(ll);
            }
        }

        queue.add(
            CreateCommand::<DECL> {
                target: UnsafeIndex::from_index(&self.0),
                phantom_sd: PhantomData,
            }
        );
    }
    fn draw<Q: CommandQueue>(&self, queue: &mut Q, parameters: DECL::Parameters,
                             primitive: Primitive, vertex_start: usize, vertex_count: usize)
    {
        struct DrawCommand<SD: ShaderDeclaration> {
            target: UnsafeIndex<GLShaderProgram>,
            parameters: SD::Parameters,
            primitive: GLenum,
            vertex_start: GLuint,
            vertex_count: GLuint,
        }

        impl<SD: ShaderDeclaration> Command for DrawCommand<SD> {
            fn get_sort_key(&self) -> usize {
                1
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                // Without the unsafe code resourecs is borrowed mutable multiple times
                // (once for target and once for the draw function call)
                // As we know (but not the compiler) that, shader won't be accessed
                // during call, it is safe to have a mutable reference into the shader
                let target = unsafe {
                    let a = &mut resources[&self.target] as *mut GLShaderProgram;
                    &mut *a
                };
                target.draw(resources, ll, &self.parameters, self.primitive, self.vertex_start, self.vertex_count);
            }
        }

        queue.add(
            DrawCommand::<DECL> {
                target: UnsafeIndex::from_index(&self.0),
                parameters: parameters,
                primitive: gl_get_primitive_enum(primitive),
                vertex_start: vertex_start as GLuint,
                vertex_count: vertex_count as GLuint,
            }
        );
    }
}

use std::marker::PhantomData;
use backend::*;

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

*/

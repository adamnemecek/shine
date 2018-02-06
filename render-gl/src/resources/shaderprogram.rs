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
        target.parse_parameters(context.ll, DECL::Parameters::get_index_by_name);
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
        {
            let target = unsafe { context.shader_program_store.at_unsafe_mut(&self.target) };
            if !target.bind(context.ll) {
                return;
            }
        }
        self.parameters.bind(context);
        context.ll.draw(self.primitive, self.vertex_start, self.vertex_count);
    }
}


pub type ShaderProgramStore = Store<GLShaderProgram>;
pub type ReadGuardShaderProgram<'a> = ReadGuard<'a, GLShaderProgram>;
pub type WriteGuardShaderProgram<'a> = WriteGuard<'a, GLShaderProgram>;
pub type ShaderProgramIndex = Index<GLShaderProgram>;
pub type UnsafeShaderProgramIndex = UnsafeIndex<GLShaderProgram>;


/// Handle to an index buffer
#[derive(Clone, Debug)]
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

        //println!("ShaderProgram - release");
        queue.add_command(0,
                          ReleaseCommand {
                              target: UnsafeIndex::from_index(&self.0),
                          });
    }
}

impl<DECL: ShaderDeclaration<PlatformEngine>> ShaderProgram<DECL, PlatformEngine> for ShaderProgramHandle<DECL> {
    fn compile(&self, queue: &mut GLCommandQueue) {
        //println!("ShaderProgram - compile");
        queue.add_command(0,
                          CreateCommand::<DECL> {
                              target: UnsafeIndex::from_index(&self.0),
                              phantom: PhantomData,
                          });
    }

    fn draw(&self, queue: &mut GLCommandQueue, parameters: DECL::Parameters, primitive: Primitive, vertex_start: usize, vertex_count: usize) {
        //println!("ShaderProgram - draw");
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

use store::fjsqueue::*;
use resources::*;


#[derive(Copy, Clone, Debug)]
pub struct CommandOrder(pub u8, pub u32);


/// Commands using dynamic dispatch
pub trait DynCommand: 'static {
    fn process(&mut self, context: &mut GLCommandProcessContext);
}

impl<T: DynCommand> From<T> for Command {
    #[inline(always)]
    fn from(value: T) -> Command {
        Command::DynamicCommand(Box::new(value))
    }
}


/// Enum for render commands.
pub enum Command {
    Hello { time: f32 },
    //VertexCreate(vertexbuffer::CreateCommand),
    VertexRelease(vertexbuffer::ReleaseCommand),
    IndexCreate(indexbuffer::CreateCommand),
    IndexRelease(indexbuffer::ReleaseCommand),
    Texture2DCreate(texture2d::CreateCommand),
    Texture2DRelease(texture2d::ReleaseCommand),
    //ShaderProgramCreate(shaderprogram::CreateCommand),
    ShaderProgramRelease(shaderprogram::ReleaseCommand),

    DynamicCommand(Box<DynCommand>),
}

impl Command {
    pub fn process(self, context: &mut GLCommandProcessContext) {
        use Command::*;
        match self {
            Hello { time } => context.ll.hello(time),
            //VertexCreate(cmd) => cmd.process(ll, flush),
            VertexRelease(cmd) => cmd.process(context),
            IndexCreate(cmd) => cmd.process(context),
            IndexRelease(cmd) => cmd.process(context),
            Texture2DCreate(cmd) => cmd.process(context),
            Texture2DRelease(cmd) => cmd.process(context),
            //ShaderProgramCreate(cmd) => cmd.process(context),
            ShaderProgramRelease(cmd) => cmd.process(context),

            DynamicCommand(mut cmd) => cmd.process(context),
        }
    }
}


pub type CommandStore = FJSQueue<CommandOrder, Command>;
pub type CommandProduceGuard<'a> = ProduceGuard<'a, CommandOrder, Command>;

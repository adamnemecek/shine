use lowlevel::*;
use store::fjsqueue::*;
use resources::*;


#[derive(Copy, Clone, Debug)]
pub struct CommandOrder(pub u8, pub u32);


/// User specific command
pub trait UserCommand : 'static{
    fn process(&mut self, ll: &mut LowLevel, flush: &mut GLFrameFlusher);
}


/// Enum for render commands.
pub enum Command {
    Hello { time: f32 },
    VertexCreate(vertexbuffer::CreateCommand),
    VertexRelease(vertexbuffer::ReleaseCommand),
    IndexCreate(indexbuffer::CreateCommand),
    IndexRelease(indexbuffer::ReleaseCommand),

    User(Box<UserCommand>),
}

impl Command {
    pub fn process(self, ll: &mut LowLevel, flush: &mut GLFrameFlusher) {
        use Command::*;
        match self {
            Hello { time } => ll.hello(time),
            VertexCreate(cmd) => cmd.process(ll, flush),
            VertexRelease(cmd) => cmd.process(ll, flush),
            IndexCreate(cmd) => cmd.process(ll, flush),
            IndexRelease(cmd) => cmd.process(ll, flush),

            User(mut cmd) => cmd.process(ll, flush),
        }
    }
}


pub type CommandStore = FJSQueue<CommandOrder, Command>;
pub type CommandProduceGuard<'a> = ProduceGuard<'a, CommandOrder, Command>;

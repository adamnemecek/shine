use lowlevel::*;
use store::fjsqueue::*;
use resources::*;


#[derive(Copy, Clone, Debug)]
pub struct CommandOrder(pub u8, pub u32);

pub enum Command {
    Hello { time: f32 },
    IndexRelease(indexbuffer::ReleaseCommand),
    IndexCreate(indexbuffer::CreateCommand),
}


impl Command {
    pub fn process(self, ll: &mut LowLevel, flush: &mut GLFrameFlush) {
        use Command::*;
        match self {
            Hello { time } => ll.hello(time),
            IndexRelease(mut cmd) => cmd.process(ll, flush),
            IndexCreate(mut cmd) => cmd.process(ll, flush),
        }
    }
}

pub type CommandStore = FJSQueue<CommandOrder, Command>;
pub type CommandProduceGuard<'a> = ProduceGuard<'a, CommandOrder, Command>;

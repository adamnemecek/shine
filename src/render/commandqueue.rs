use render::*;

pub struct CommandQueue {
    pub platform: CommandQueueImpl
}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue { platform: CommandQueueImpl::new() }
    }
}

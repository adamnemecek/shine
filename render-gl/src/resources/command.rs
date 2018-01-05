use core::*;
use resources::indexbuffer;

pub enum GLCommand {
    IndexRelease(indexbuffer::ReleaseCommand),
    IndexCreate(indexbuffer::CreateCommand),
}

impl Command for GLCommand {
    fn get_sort_key(&self) -> usize {
        0
    }
}

/*
impl GLCommand {
    fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        let target = &mut resources[&self.target];
        target.upload_data(ll, self.type_id, self.data.as_slice());
    }
}*/
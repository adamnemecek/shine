use container::ops::At;

use render::*;
use render::opengl::window::window::*;

struct CommandSortKey(PassMetaIndex, usize);

/// Trait for each render command
pub trait Command: 'static {
    fn get_sort_key(&self) -> usize;
    fn process(&mut self, ll: &mut LowLevel);
}


/// Structure to store an manage GLCommand
pub struct CommandStore {
    commands: Vec<Box<Command>>,
    sort: Vec<(CommandSortKey, usize)>,
}

impl CommandStore {
    pub fn new() -> CommandStore {
        CommandStore {
            commands: vec!(),
            sort: vec!(),
        }
    }

    pub ( crate ) fn add<C: Command>(&mut self, sort_key: (PassMetaIndex, usize), cmd: C) {
        self.sort.push((CommandSortKey(sort_key.0, sort_key.1), self.commands.len()));
        self.commands.push(Box::new(cmd));
    }

    pub ( crate ) fn clear(&mut self) {
        self.sort.clear();
        self.commands.clear();
    }

    pub ( crate ) fn sort<V: At<PassMetaIndex, Output=usize>>(&mut self, view_order: &V) {
        self.sort.sort_by(
            |ref a, ref b| {
                let ref a = a.0;
                let ref b = b.0;
                let va = view_order.at(a.0);
                let vb = view_order.at(b.0);
                if va != vb { va.partial_cmp(&vb).unwrap() } else { a.1.partial_cmp(&b.1).unwrap() }
            });
    }

    pub ( crate ) fn process(&mut self, window: &GLWindow) {
        for sorted_item in self.sort.iter() {
            let ref mut cmd = self.commands[sorted_item.1];
            window.process(|ll| cmd.process(ll));
        }
    }
}

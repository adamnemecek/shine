use resources::*;
use manager::*;

struct CommandSortKey(ActivePassIndex, usize);

/// Structure to store an manage Commands using storing boxed instances in a vector
pub struct CommandStore<CMD: Command> {
    commands: Vec<CMD>,
    sort: Vec<(CommandSortKey, usize)>,
}

impl<CMD: Command> CommandStore<CMD> {
    pub fn new() -> CommandStore<CMD> {
        CommandStore {
            commands: vec!(),
            sort: vec!(),
        }
    }

    pub fn add(&mut self, pass: ActivePassIndex, cmd: CMD) {
        self.sort.push((CommandSortKey(pass, cmd.get_sort_key()), self.commands.len()));
        self.commands.push(cmd);
    }

    pub fn clear(&mut self) {
        self.sort.clear();
        self.commands.clear();
    }

    pub fn sort<F: Fn(ActivePassIndex) -> usize>(&mut self, get_order: &F) {
        self.sort.sort_by(
            |ref a, ref b| {
                let ref a = a.0;
                let ref b = b.0;
                let va = get_order(a.0);
                let vb = get_order(b.0);
                if va != vb { va.partial_cmp(&vb).unwrap() } else { a.1.partial_cmp(&b.1).unwrap() }
            });
    }

    /* pub fn process<'a>(&mut self, window: &mut GLWindow, resources: &mut GuardedResources<'a>) {
         let ll = window.get_ll();

         // handle new resource alocations
         resources.vertex_buffers.process_requests();
         resources.index_buffers.process_requests();
         resources.textures_2d.process_requests();
         resources.shaders.process_requests();

         // process commands
         for sorted_item in self.sort.iter() {
             let ref mut cmd = self.commands[sorted_item.1];
             cmd.process(resources, ll);
         }

         // release unused allocations
         resources.vertex_buffers.drain_unused(|e| {
             e.release(ll);
             true
         });
         resources.index_buffers.drain_unused(|e| {
             e.release(ll);
             true
         });
         resources.textures_2d.drain_unused(|e| {
             e.release(ll);
             true
         });
         resources.shaders.drain_unused(|e| {
             e.release(ll);
             true
         });
     }*/
}

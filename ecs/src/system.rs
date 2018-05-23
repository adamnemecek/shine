use shred::Resources;

pub struct System {
    resources: Resources,
}

impl System {
    pub fn new() -> System {
        System {
            resources: Resources::new(),
        }
    }
}
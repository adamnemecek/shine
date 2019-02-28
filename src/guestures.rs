pub struct Guestures {
    pub side: f32,
    pub up: f32,
    pub forward: f32,
    pub roll: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Guestures {
    pub fn new() -> Guestures {
        Guestures {
            side: 0.,
            up: 0.,
            forward: 0.,
            roll: 0.,
            yaw: 0.,
            pitch: 0.,
        }
    }
}

impl Default for Guestures {
    fn default() -> Self {
        Guestures::new()
    }
}

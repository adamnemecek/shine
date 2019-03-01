use std::collections::HashMap;

pub struct State {
    /// value of analog joytic for each axis
    joytick: HashMap<u32, f32>,

    /// time of pressing for each pressed button
    button: HashMap<u32, u64>,
}

impl State {
    pub fn new() -> State {
        State {
            joytick: HashMap::new(),
            button: HashMap::new(),
        }
    }

    pub fn set_joystick(&mut self, axis_id: u32, value: f32) {
        let value = if value > 1. {
            1.
        } else if value < -1. {
            -1.
        };

        if value == 0 {
            let _ = self.joytick.remove(axis_id);
        } else {
            let mut entry = self.joytick.entry(axis_id);
            entry.or_insert(0.) = value;
        }
    }

    pub fn get_joystick(&self, axis_id: u32) -> f32 {
        self.joytick.get(&axis_id).unwrap_or(0.)
    }

    pub fn set_pressed(&mut self, button_id: u32, time: u64) {
        let mut entry = self.button.entry(button_id);
        entry.or_insert(0) = time;
    }

    pub fn set_released(&mut self, button_id: u32) {
        let _ = self.button.remove(&button_id);
    }

    pub fn get_press_time(&mut self, button_id: u32) -> u64 {
        self.button.get(&button_id).unwrap_or(0)
    }

    pub fn is_pressed(&mut self, button_id: u32, time: u64) -> bool {
        time - self.get_press_time(button_id) > 0
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

use std::collections::HashMap;
use std::slice::Iter;
use std::vec::Drain;

pub enum Event {
    ButtonClick(u32),
    ButtonClickWithPosition(u32, (f32, f32)),
}

#[derive(Default, Debug)]
struct JoystickState {
    /// Reset joystick automatically in prepare pass (ex: used for mouse move to reset axis when mouse is not moved)
    auto_reset: bool,

    /// Current value in the [-1,1] range
    value: f32,
}

#[derive(Default, Debug)]
struct ButtonState {
    /// Time of the press
    down_time: u128,
}

pub struct State {
    /// Value of the analog joystic for each axis in the [-1,1] range.
    joysticks: HashMap<u32, JoystickState>,

    /// Time of pressure for each pressed button.
    buttons: HashMap<u32, ButtonState>,

    /// Events for non-state managed input triggers.
    events: Vec<Event>,
}

impl State {
    pub fn new() -> State {
        State {
            joysticks: HashMap::new(),
            buttons: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.joysticks.clear();
        self.buttons.clear();
        self.events.clear();
    }

    pub fn auto_reset_joystick(&mut self) {
        self.joysticks.retain(|_, v| !v.auto_reset);
    }

    pub fn get_joystick(&self, axis_id: u32) -> f32 {
        match self.joysticks.get(&axis_id) {
            None => 0.,
            Some(ref s) => s.value,
        }
    }

    pub fn set_joystick(&mut self, axis_id: u32, value: f32, auto_reset: bool) {
        // clamp input to [-1,1]
        let value = if value > 1. {
            1.
        } else if value < -1. {
            -1.
        } else {
            value
        };

        if value == 0. {
            let _ = self.joysticks.remove(&axis_id);
        } else {
            let entry = self.joysticks.entry(axis_id);
            let mut state = entry.or_insert(JoystickState::default());
            state.value = value;
            state.auto_reset = auto_reset;
        }
    }

    pub fn get_press_time(&mut self, button_id: u32) -> u128 {
        match self.buttons.get(&button_id) {
            None => 0,
            Some(v) => v.down_time,
        }
    }

    pub fn is_pressed(&mut self, button_id: u32) -> bool {
        match self.buttons.get(&button_id) {
            None => false,
            Some(v) => true,
        }
    }

    pub fn set_pressed(&mut self, button_id: u32, time: u128) {
        let entry = self.buttons.entry(button_id);
        let state = entry.or_insert(ButtonState::default());
        state.down_time = time;
    }

    pub fn set_released(&mut self, button_id: u32) {
        let _ = self.buttons.remove(&button_id);
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn peek_events(&mut self) -> Iter<'_, Event> {
        self.events.iter()
    }

    pub fn poll_events(&mut self) -> Drain<'_, Event> {
        self.events.drain(..)
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

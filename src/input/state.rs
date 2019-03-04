use crate::input::{AxisId, ButtonId};
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
    /// The last update time
    pub(crate) time: u128,

    /// Cursore poisition on the normalize [0,1]^2 screen
    pub(crate) cursore_position: (f32, f32),

    /// Value of the analog joystic for each axis in the [-1,1] range.
    joysticks: HashMap<AxisId, JoystickState>,

    /// Time of pressure for each pressed button.
    buttons: HashMap<ButtonId, ButtonState>,

    /// Events for non-state managed input triggers.
    events: Vec<Event>,
}

impl State {
    pub fn new() -> State {
        State {
            time: 0,
            cursore_position: (0., 0.),
            joysticks: HashMap::new(),
            buttons: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.joysticks.clear();
        self.buttons.clear();
        self.events.clear();
        self.time = 0;
    }

    pub fn time(&self) -> u128 {
        self.time
    }

    pub fn cursore_position(&self) -> (f32, f32) {
        self.cursore_position
    }

    pub fn auto_reset_joystick(&mut self) {
        self.joysticks.retain(|_, v| !v.auto_reset);
    }

    pub fn get_joystick(&self, axis_id: AxisId) -> f32 {
        match self.joysticks.get(&axis_id) {
            None => 0.,
            Some(ref s) => s.value,
        }
    }

    pub fn set_joystick(&mut self, axis_id: AxisId, value: f32, auto_reset: bool) {
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

    pub fn get_press_time(&mut self, button_id: &ButtonId) -> u128 {
        match self.buttons.get(button_id) {
            None => 0,
            Some(v) => v.down_time,
        }
    }

    pub fn is_pressed(&mut self, button_id: &ButtonId) -> bool {
        match self.buttons.get(button_id) {
            None => false,
            Some(v) => true,
        }
    }

    pub fn set_pressed(&mut self, button_id: ButtonId, is_pressed: bool) {
        if is_pressed {
            let entry = self.buttons.entry(button_id);
            let state = entry.or_insert(ButtonState::default());
            state.down_time = self.time;
        } else {
            let _ = self.buttons.remove(&button_id);
        }
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

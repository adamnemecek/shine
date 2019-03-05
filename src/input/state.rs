use crate::input::AxisId;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct JoystickState {
    /// Reset joystick automatically in prepare pass (ex: used for mouse move to reset axis when mouse is not moved)
    autoreset: bool,

    /// Required modifier mask
    modifier_mask: ModifierFilterMask,

    /// Current value in the [-1,1] range
    value: f32,
}

pub struct State {
    /// The last update time
    pub(crate) time: u128,

    /// Cursore poisition on the normalize [0,1]^2 screen
    pub(crate) cursore_position: (f32, f32),

    /// Current modifiers
    modifiers: u128,

    /// Autoreset modifiers
    autoreset_modifiers: u128,

    /// Value of the analog joystic for each axis in the [-1,1] range.
    joysticks: HashMap<AxisId, JoystickState>,
}

impl State {
    pub fn new() -> State {
        State {
            time: 0,
            cursore_position: (0., 0.),
            joysticks: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.joysticks.clear();
        self.time = 0;
    }

    pub fn get_time(&self) -> u128 {
        self.time
    }

    pub fn get_cursore_position(&self) -> (f32, f32) {
        self.cursore_position
    }

    pub fn autoreset_modifiers(&mut self) {
        self.mask = self.mask & !self.autoreset_modifiers();
    }

    pub fn is_modifiers(&self, modifier_id: ModifierId) -> bool {
        let m = 0_u128 << modifier_id.id();
        (self.modifiers & m) != 0
    }

    pub fn set_modifiers(&self, modifier_id: ModifierId, autoreset: bool) {
        let m = 0_u128 << modifier_id.id()
        self.mask |= m;
        if autoreset {
            self.autoreset_modifiers |= m;
        } else {
            self.autoreset_modifiers &= !m;
        }
    }

    pub fn autoreset_joystick(&mut self) {
        self.joysticks.retain(|_, v| !v.autoreset);
    }

    pub fn get_joystick(&self, axis_id: AxisId) -> f32 {
        match self.joysticks.get(&axis_id) {
            None => 0.,
            Some(ref s) => s.value,
        }
    }

    pub fn is_joystick(&self, axis_id: AxisId) -> bool {
        self.joysticks.get(&axis_id).is_some()
    }

    pub fn set_joystick(&mut self, axis_id: AxisId, value: f32, autoreset: bool) {
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
            state.autoreset = autoreset;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

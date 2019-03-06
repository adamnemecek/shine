use crate::input::AxisId;
use crate::input::{ModifierFilterMask, ModifierId, ModifierMask};
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
    pub(crate) time: u128,                        // The last update time
    pub(crate) cursore_position: (f32, f32),      // Cursore poisition on the normalize [0,1]^2 screen
    pub(crate) modifier_mask: ModifierMask,       // Current modifiers
    pub(crate) autoreset_modifiers: ModifierMask, // Autoreset modifiers

    /// Value of the analog joystic for each axis in the [-1,1] range.
    joysticks: HashMap<AxisId, JoystickState>,
}

impl State {
    pub fn new() -> State {
        State {
            time: 0,
            cursore_position: (0., 0.),
            modifier_mask: ModifierMask::default(),
            autoreset_modifiers: ModifierMask::default(),
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
        self.modifier_mask.clear_masked(&self.autoreset_modifiers);
    }

    pub fn is_modifier(&self, modifier_id: ModifierId) -> bool {
        self.modifier_mask.get(modifier_id)
    }

    pub fn set_modifier(&mut self, modifier_id: ModifierId, pressed: bool, autoreset: bool) {
        self.modifier_mask.set(modifier_id, pressed);
        self.autoreset_modifiers.set(modifier_id, autoreset);
    }

    pub fn autoreset_joystick(&mut self) {
        self.joysticks.retain(|_, v| !v.autoreset);
    }

    pub fn get_joystick(&self, axis_id: AxisId) -> f32 {
        match self.joysticks.get(&axis_id) {
            None => 0.,
            Some(ref s) => {
                if s.modifier_mask.check(&self.modifier_mask) {
                    s.value
                } else {
                    0.
                }
            }
        }
    }

    pub fn is_joystick(&self, axis_id: AxisId) -> bool {
        self.get_joystick(axis_id) != 0.
    }

    pub fn set_joystick(&mut self, axis_id: AxisId, modifier_mask: ModifierFilterMask, value: f32, autoreset: bool) {
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
            state.modifier_mask = modifier_mask;
            state.autoreset = autoreset;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

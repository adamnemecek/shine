use crate::input::ButtonId;
use crate::input::{ModifierFilterMask, ModifierId, ModifierMask};
use std::collections::HashMap;

/// State of a button
#[derive(Clone, Default, Debug)]
struct ButtonState {
    /// Reset button automatically in prepare pass (ex: used for mouse move to reset axis when mouse is not moved)
    autoreset: bool,

    /// Required modifier mask
    modifier_mask: ModifierFilterMask,

    /// Current value in the [-1,1] range
    value: f32,
}

/// Store the current input state
pub struct State {
    time: u128,                              // The last update time
    cursore_position: (f32, f32),            // Cursore poisition on the normalize [0,1]^2 screen
    modifier_mask: ModifierMask,             // Current modifiers
    autoreset_modifiers: ModifierMask,       // Autoreset modifiers
    buttons: HashMap<ButtonId, ButtonState>, // State of the buttons
}

impl State {
    pub fn new() -> State {
        State {
            time: 0,
            cursore_position: (0., 0.),
            modifier_mask: ModifierMask::default(),
            autoreset_modifiers: ModifierMask::default(),
            buttons: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.modifier_mask.clear();
        self.autoreset_modifiers.clear();
        self.time = 0;
        self.cursore_position = (0., 0.);
    }

    /// Copy the previous state
    pub fn prepare(&mut self, prev: &State, time: u128) {
        self.clear();

        self.time = time;

        self.modifier_mask = ModifierMask::from_masked_clear(&prev.modifier_mask, &prev.autoreset_modifiers);

        self.buttons.clear();
        for (k, j) in prev.buttons.iter() {
            if j.autoreset {
                continue;
            }

            self.buttons.insert(*k, j.clone());
        }
    }

    pub fn get_time(&self) -> u128 {
        self.time
    }

    pub fn set_modifier(&mut self, modifier_id: ModifierId, pressed: bool, autoreset: bool) {
        self.modifier_mask.set(modifier_id, pressed);
        self.autoreset_modifiers.set(modifier_id, autoreset);
    }

    pub fn is_modifier(&self, modifier_id: ModifierId) -> bool {
        self.modifier_mask.get(modifier_id)
    }

    pub fn set_button(&mut self, button_id: ButtonId, modifier_mask: ModifierFilterMask, value: f32, autoreset: bool) {
        // clamp input to [-1,1]
        let value = if value > 1. {
            1.
        } else if value < -1. {
            -1.
        } else {
            value
        };

        if value == 0. {
            let _ = self.buttons.remove(&button_id);
        } else {
            let entry = self.buttons.entry(button_id);
            let mut state = entry.or_insert(ButtonState::default());
            state.value = value;
            state.modifier_mask = modifier_mask;
            state.autoreset = autoreset;
        }
    }

    pub fn get_button(&self, button_id: ButtonId) -> f32 {
        match self.buttons.get(&button_id) {
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

    pub fn is_button(&self, button_id: ButtonId) -> bool {
        self.get_button(button_id) != 0.
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

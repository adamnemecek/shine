use crate::{ButtonId, Guesture, ModifierFilterMask, State};
use std::any::Any;

pub struct KeyboardJoystick {
    axis: Vec<(ButtonId, ButtonId, ButtonId, ModifierFilterMask)>,
}

impl KeyboardJoystick {
    pub fn new() -> KeyboardJoystick {
        KeyboardJoystick { axis: Vec::new() }
    }

    pub fn add_axis(
        &mut self,
        button_pos: ButtonId,
        button_neg: ButtonId,
        axis_button: ButtonId,
        axis_modifiers: Option<ModifierFilterMask>,
    ) {
        let mask = axis_modifiers.unwrap_or_else(ModifierFilterMask::default);
        self.axis.push((button_pos, button_neg, axis_button, mask));
    }
}

impl Default for KeyboardJoystick {
    fn default() -> KeyboardJoystick {
        KeyboardJoystick::new()
    }
}

impl Guesture for KeyboardJoystick {
    fn as_any(&self) -> &Any {
        self
    }

    fn on_update(&mut self, _prev_state: &State, state: &mut State) {
        for (pos, neg, axis, axis_modifiers) in &self.axis {
            let is_pos = state.is_button(*pos);
            let is_neg = state.is_button(*neg);
            if is_pos == is_neg {
                state.remove_button(*axis);
            } else if is_pos {
                state.set_button(*axis, axis_modifiers.clone(), 1., false);
            } else {
                assert!(is_neg);
                state.set_button(*axis, axis_modifiers.clone(), -1., false);
            }
        }
    }
}

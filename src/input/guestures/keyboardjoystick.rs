use crate::input::{GuestureHandler, GuestureResponse, State};

struct Control {
    /// Button that emulates the moves along the positive direction
    inc_button_id: u32,

    /// Button that emulates the moves along the negative direction
    dec_button_id: u32,

    /// The emulated axis
    axis: u32,
}

/// Emulate joystick by converting keybord down/up events into axis events
pub struct KeyboardJoystick {
    axis: Vec<Control>,
}

impl KeyboardJoystick {
    pub fn new() -> KeyboardJoystick {
        KeyboardJoystick { axis: Vec::new() }
    }

    pub fn add_axis(&mut self, axis: u32, inc_button_id: u32, dec_button_id: u32) {
        self.axis.push(Control {
            inc_button_id,
            dec_button_id,
            axis,
        });
    }
}

impl GuestureHandler for KeyboardJoystick {
    fn on_prepare(&mut self, _time: u128, _state: &mut State) {}

    fn on_update(&mut self, _time: u128, _state: &mut State) {}

    fn on_joystick(&mut self, _time: u128, _state: &mut State, _axis_id: u32, _value: f32) -> GuestureResponse {
        GuestureResponse::None
    }

    fn on_button(&mut self, _time: u128, _state: &mut State, _button_id: u32, _is_down: bool) -> GuestureResponse {
        GuestureResponse::None
    }
}

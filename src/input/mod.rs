pub mod guestures;
mod manager;
mod mapping;
mod state;

pub use self::manager::*;
pub use self::state::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ButtonId(u32);

impl ButtonId {
    pub fn new(code: u32) -> ButtonId {
        ButtonId(code)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AxisId(u32);

impl AxisId {
    pub fn new(code: u32) -> AxisId {
        AxisId(code)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GuestureResponse {
    None,
    Consumed,
}

pub trait GuestureHandler: Send + Sync {
    /// Called before injecting system messages
    fn on_prepare(&mut self, _state: &mut State);

    /// Called after the injection of system messages
    fn on_update(&mut self, _state: &mut State);

    /// Raw keyboard input
    fn on_raw_keyboard(&mut self, _state: &mut State, key: &winit::KeyboardInput);

    /// Called during the handling of the button events of the system
    fn on_button(&mut self, _state: &mut State, _button_id: ButtonId, _is_down: bool) -> GuestureResponse;

    /// Called during the handling of the joystick events of the system
    fn on_joystick(&mut self, _state: &mut State, _axis_id: AxisId, _value: f32) -> GuestureResponse;
}

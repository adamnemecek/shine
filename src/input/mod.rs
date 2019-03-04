pub mod guestures;
mod manager;
mod state;

pub use self::manager::*;
pub use self::state::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GuestureResponse {
    None,
    Consumed,
}

pub trait GuestureHandler: Send + Sync {
    /// Called before injecting system messages
    fn on_prepare(&mut self, _time: u128, _state: &mut State);

    /// Called after the injection of system messages
    fn on_update(&mut self, _time: u128, _state: &mut State);

    /// Called during the handling of the joystick events of the system
    fn on_joystick(&mut self, _time: u128, _state: &mut State, _axis_id: u32, _value: f32) -> GuestureResponse;

    /// Called during the handling of the button events of the system
    fn on_button(&mut self, _time: u128, _state: &mut State, _button_id: u32, _is_down: bool) -> GuestureResponse;
}

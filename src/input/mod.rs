use crate::input::State;

mod manager;
mod state;

pub use self::manager::*;
pub use self::state::*;

pub enum GuestureResponse {
    None,
    Consumed,
}

pub trait GuestureHandler : Send + Sync {
    fn on_joystick(&mut self, state: &mut State, axis_id: u32, value: f32) -> GuestureResponse;
    fn on_button(&mut self, state: &mut State, button_id: u32, is_down: bool, time: f32) -> GuestureResponse;
}

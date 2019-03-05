pub mod guestures;
mod manager;
mod mapping;
mod modifiermask;
mod state;

pub use self::manager::*;
pub use self::mapping::InputMapping;
pub use self::modifiermask::*;
pub use self::state::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AxisId(u32);

impl AxisId {
    pub const fn new(code: u32) -> AxisId {
        AxisId(code)
    }

    pub fn id() -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModifierId(u32);

impl ModifierId {
    pub const fn new(code: u32) -> ModifierId {
        ModifierId(code)
    }

    pub fn id() -> u32 {
        self.0
    }
}

pub trait GuestureHandler: Send + Sync {
    /// Called before injecting system messages
    fn on_prepare(&mut self, state: &mut State);

    /// Called after the injection of system messages
    fn on_update(&mut self, state: &mut State);

    /// Called during the handling of the system messages
    fn on_joystick(&mut self, state: &mut State, axis_id: AxisId, value: f32);
}

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
pub struct ButtonId(u32);

impl ButtonId {
    pub const fn new(code: u32) -> ButtonId {
        ButtonId(code)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModifierId(u32);

impl ModifierId {
    pub const fn new(code: u32) -> ModifierId {
        //assert!(code < modifiermask::MAX_MODIFIER_COUNT);
        ModifierId(code)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

pub trait GuestureHandler: Send + Sync {
    fn on_update(&mut self, prev_state: &State, state: &mut State);
}

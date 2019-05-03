pub mod guestures;
pub use self::guestures::Guesture;

mod manager;
pub use self::manager::*;

mod mapping;
pub use self::mapping::InputMapping;

mod modifiermask;
pub use self::modifiermask::*;

mod state;
pub use self::state::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ButtonId(u32);

impl ButtonId {
    pub const fn new(code: u32) -> ButtonId {
        ButtonId(code)
    }

    pub fn id(self) -> u32 {
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

    pub fn id(self) -> u32 {
        self.0
    }
}

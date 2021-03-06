use crate::ModifierId;

pub const MAX_MODIFIER_COUNT: u32 = 128;

#[derive(Clone, Debug)]
pub struct ModifierMask(pub(crate) u128);

impl ModifierMask {
    pub fn new() -> ModifierMask {
        ModifierMask(0)
    }

    pub fn new_from(filter: &[ModifierId]) -> ModifierMask {
        let mut mask = ModifierMask(0);
        mask.set_from_slice(filter);
        mask
    }

    pub fn from_masked_clear(mask: &ModifierMask, clear_mask: &ModifierMask) -> ModifierMask {
        ModifierMask(mask.0 & !clear_mask.0)
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn get(&self, modifier: ModifierId) -> bool {
        let m = 1_u128 << modifier.id();
        (self.0 & m) != 0
    }

    pub fn set(&mut self, modifier: ModifierId, value: bool) {
        let m = 1_u128 << modifier.id();
        if value {
            self.0 |= m;
        } else {
            self.0 &= !m;
        }
    }

    pub fn set_from_slice(&mut self, filter: &[ModifierId]) {
        for i in filter {
            let m = 1_u128 << i.id();
            self.0 |= m;
        }
    }
}

impl Default for ModifierMask {
    fn default() -> ModifierMask {
        ModifierMask::new()
    }
}

impl From<&[ModifierId]> for ModifierMask {
    fn from(value: &[ModifierId]) -> ModifierMask {
        ModifierMask::new_from(value)
    }
}

impl From<[ModifierId; 1]> for ModifierMask {
    fn from(value: [ModifierId; 1]) -> ModifierMask {
        ModifierMask::new_from(&value)
    }
}

#[derive(Clone, Debug)]
pub struct ModifierFilterMask {
    /// Tha mask to consider in the required mask
    filter: ModifierMask,

    /// The required value for each modifiers
    required: ModifierMask,
}

impl ModifierFilterMask {
    pub fn new() -> ModifierFilterMask {
        ModifierFilterMask {
            filter: ModifierMask::new(),
            required: ModifierMask::new(),
        }
    }

    pub fn new_from(filter: &[(ModifierId, bool)]) -> ModifierFilterMask {
        let mut mask = ModifierFilterMask::new();

        for (i, f) in filter {
            log::trace!("{:?}", mask.filter);
            assert!(!mask.filter.get(*i), "filter already set for {:?}", i);
            mask.filter.set(*i, true);
            mask.required.set(*i, *f);
        }
        mask
    }

    pub fn check(&self, mask: &ModifierMask) -> bool {
        (mask.0 & self.filter.0) == self.required.0
    }
}

impl Default for ModifierFilterMask {
    fn default() -> ModifierFilterMask {
        ModifierFilterMask::new()
    }
}

impl From<&[(ModifierId, bool)]> for ModifierFilterMask {
    fn from(value: &[(ModifierId, bool)]) -> ModifierFilterMask {
        ModifierFilterMask::new_from(value)
    }
}

impl From<[(ModifierId, bool); 1]> for ModifierFilterMask {
    fn from(value: [(ModifierId, bool); 1]) -> ModifierFilterMask {
        ModifierFilterMask::new_from(&value)
    }
}

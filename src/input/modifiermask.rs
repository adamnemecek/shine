
const MAX_MODIFIER_COUNT: u32 = 128;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifierFilter {
    Ignore,
    Required,
    Fail,
}

pub type ModifierMask = u128;

impl ModifierMask {
    pub fn new() -> ModifierMask {
        ModifierMask { mask: 0 }
    }

    pub fn set(u32: Id, )]) -> ModifierMask {
        let mask = ModifierMask::new();
        for (f, i) in filter {
            let m = 1_u128 << i;
            assert_eq!((mask.filter & m), 0, "filter already set for {}", i);
            mask.filter |= m;
            if *f == ModifierFilter::Required {
                mask.required |= m;
            }
        }
        mask
    }
}

pub struct ModifierFilterMask {
    /// Un-ignored modifiers
    filter: u128,

    /// The required modifiers
    required: u128,
}

impl ModifierFilterMask {
    pub fn new() -> ModifierFilterMask {
        ModifierFilterMask { filter: 0, required: 0 }
    }

    pub fn from(filter: &[(ModifierFilter, u32)]) -> ModifierFilterMask {
        let mask = ModifierMask::new();
        for (f, i) in filter {
            let m = 1_u128 << i;
            assert_eq!((mask.filter & m), 0, "filter already set for {}", i);
            mask.filter |= m;
            if *f == ModifierFilter::Required {
                mask.required |= m;
            }
        }
        mask
    }
}
//! Section-header table index.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u16);

impl Index {
    pub const UNDEFINED: Self = Self(0);
    pub const LOWER_RESERVED: Self = Self(0xff00);
    pub const LOWER_PROCESSOR: Self = Self(0xff00);
    pub const UPPER_PROCESSOR: Self = Self(0xff1f);
    pub const LOWER_OPERATING_SYSTEM: Self = Self(0xff20);
    pub const UPPER_OPERATING_SYSTEM: Self = Self(0xff3f);
    pub const ABSOLUTE: Self = Self(0xfff1);
    pub const COMMON: Self = Self(0xfff2);
    pub const EXTENDED: Self = Self(0xffff);
    pub const UPPER_RESERVED: Self = Self(0xffff);

    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

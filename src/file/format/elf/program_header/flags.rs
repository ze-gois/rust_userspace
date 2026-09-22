//! Segment permission and processor/OS flag bits (`p_flags`).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags(u32);

impl Flags {
    pub const EXECUTE: u32 = 0x1;
    pub const WRITE: u32 = 0x2;
    pub const READ: u32 = 0x4;
    pub const OPERATING_SYSTEM_MASK: u32 = 0x0ff0_0000;
    pub const PROCESSOR_MASK: u32 = 0xf000_0000;

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn executable(self) -> bool {
        self.0 & Self::EXECUTE != 0
    }

    pub const fn writable(self) -> bool {
        self.0 & Self::WRITE != 0
    }

    pub const fn readable(self) -> bool {
        self.0 & Self::READ != 0
    }
}

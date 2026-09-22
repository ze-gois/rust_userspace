//! Section attribute flags (`sh_flags`).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags(u64);

impl Flags {
    pub const WRITE: u64 = 0x1;
    pub const ALLOCATE: u64 = 0x2;
    pub const EXECUTABLE_INSTRUCTIONS: u64 = 0x4;
    pub const MERGE: u64 = 0x10;
    pub const STRINGS: u64 = 0x20;
    pub const INFORMATION_LINK: u64 = 0x40;
    pub const LINK_ORDER: u64 = 0x80;
    pub const OPERATING_SYSTEM_NONCONFORMING: u64 = 0x100;
    pub const GROUP: u64 = 0x200;
    pub const THREAD_LOCAL_STORAGE: u64 = 0x400;
    pub const COMPRESSED: u64 = 0x800;
    pub const OPERATING_SYSTEM_MASK: u64 = 0x0ff0_0000;
    pub const PROCESSOR_MASK: u64 = 0xf000_0000;

    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }

    pub const fn contains(self, flag: u64) -> bool {
        self.0 & flag != 0
    }
}

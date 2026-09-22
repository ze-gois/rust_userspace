//! Dynamic object state flags (`DT_FLAGS` / `DF_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags(u64);

impl Flags {
    /// `DF_ORIGIN`: $ORIGIN processing is required.
    pub const ORIGIN: u64 = 0x1;
    /// `DF_SYMBOLIC`: symbolic symbol resolution is required.
    pub const SYMBOLIC: u64 = 0x2;
    /// `DF_TEXTREL`: text relocations exist.
    pub const TEXT_RELOCATION: u64 = 0x4;
    /// `DF_BIND_NOW`: non-lazy binding is required.
    pub const BIND_NOW: u64 = 0x8;
    /// `DF_STATIC_TLS`: the object uses the static thread-local-storage model.
    pub const STATIC_THREAD_LOCAL_STORAGE: u64 = 0x10;

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

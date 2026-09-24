//! Processor-specific relocation type.
//!
//! The gABI defines the location of the relocation type within `r_info`,
//! while the meaning of each value belongs to the processor psABI.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type(u32);

impl Type {
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

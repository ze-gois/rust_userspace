//! Required architecture (`e_machine`).
//!
//! Assigned machine values are maintained by the gABI. This type preserves
//! the assigned numeric identity without pretending the generic ELF layer owns
//! processor-specific semantics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Machine(u16);

impl Machine {
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

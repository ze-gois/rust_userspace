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

    /// Whether this value is assigned by the current gABI machine registry.
    pub const fn is_assigned(self) -> bool {
        !matches!(
            self.0,
            11..=14
                | 16
                | 24..=35
                | 121..=130
                | 145..=159
                | 182
                | 184
                | 225..=242
                | 270..=u16::MAX
        )
    }
}

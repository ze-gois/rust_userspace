//! ELF dynamic section entries.

pub mod class_32;
pub mod class_64;
pub mod flags;
pub mod section;
pub mod tag;
pub mod validation;

pub use flags::Flags;
pub use tag::{PayloadKind, Tag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dynamic {
    pub tag: Tag,
    pub payload: u64,
}

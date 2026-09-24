//! ELF data representation.
//!
//! The gABI defines class-specific scalar types used by serialized ELF
//! structures. Class is expressed by the module namespace rather than by
//! abbreviating every Rust identifier.

pub mod decoder;
pub use decoder::Decoder;

pub mod class_32 {
    pub type Address = u32;
    pub type Offset = u32;
    pub type Half = u16;
    pub type Word = u32;
    pub type Sword = i32;
}

pub mod class_64 {
    pub type Address = u64;
    pub type Offset = u64;
    pub type Half = u16;
    pub type Word = u32;
    pub type Sword = i32;
    pub type Xword = u64;
    pub type Sxword = i64;
}

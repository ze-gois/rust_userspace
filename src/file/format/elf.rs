//! Executable and Linking Format (ELF).
//!
//! This module models the ELF object-file format described by the System V
//! Generic ABI. Operating-system loading and process construction belong to
//! consumers of this representation, not to the format itself.

pub mod compression;
pub mod header;
pub mod identification;
pub mod program_header;
pub mod representation;
pub mod section_header;
pub mod dynamic;
pub mod hash;
pub mod note;
pub mod relocation;
pub mod section;
pub mod segment;
pub mod string_table;
pub mod symbol;
pub mod object_file;
pub use object_file::{ObjectFile, ParseError};
pub mod symbol_table;

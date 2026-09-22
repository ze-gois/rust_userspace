//! Executable and Linking Format (ELF).
//!
//! This module models the ELF object-file format described by the System V
//! Generic ABI. Operating-system loading and process construction belong to
//! consumers of this representation, not to the format itself.

pub mod header;
pub mod identification;
pub mod program_header;
pub mod representation;
pub mod section_header;

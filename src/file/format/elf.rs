//! Executable and Linking Format (ELF).
//!
//! This module models the ELF object-file format described by the System V
//! Generic ABI. Operating-system loading and process construction belong to
//! consumers of this representation, not to the format itself.

// Serialized representation and global object-file description.
pub mod representation;
pub mod identification;
pub mod header;

// Execution view.
pub mod program_header;
pub mod segment;
pub mod loadable_segment;
pub mod program_interpreter;
pub mod program_header_table_image;
pub mod thread_local_storage;
pub mod base_address;
pub mod memory_image;
pub mod program_image;

// Processor-specific ELF semantics (psABI / architecture ABI).
pub mod processor_specific;

// Linking view and section contents.
pub mod section_header;
pub mod section;
pub mod section_link;
pub mod string_table;
pub mod symbol;
pub mod symbol_table;
pub mod relocation;
pub mod relocation_table;
pub mod dynamic;
pub mod dynamic_array;
pub mod hash;
pub mod initialization_termination;
pub mod note;
pub mod note_table;
pub mod compression;
pub mod section_group;

// Relationships discovered through dynamic linking information.
pub mod dynamic_symbol_table;
pub mod dynamic_hash_table;
pub mod dynamic_relocation_table;
pub mod shared_object_dependencies;

// Whole-file context and cross-structure resolution.
pub mod object_file;

pub use object_file::{ObjectFile, ParseError};

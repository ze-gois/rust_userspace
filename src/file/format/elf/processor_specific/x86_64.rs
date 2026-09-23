//! ELF semantics specific to the x86-64 processor ABI.

use crate::file::format::elf::header::Machine;

pub mod entry_point;
pub mod relocation;
pub mod stack;

/// ELF `EM_X86_64`.
pub const MACHINE: Machine = Machine::from_raw(62);

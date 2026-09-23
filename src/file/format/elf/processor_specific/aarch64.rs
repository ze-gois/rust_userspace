//! ELF semantics specific to the Arm 64-bit architecture ABI.

use crate::file::format::elf::header::Machine;

/// ELF `EM_AARCH64`.
pub const MACHINE: Machine = Machine::from_raw(183);

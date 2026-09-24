//! x86-64 ELF relocation semantics from the AMD64 psABI.

use crate::file::format::elf::{
    base_address::BaseAddress,
    memory_image::{self, MemoryImageWriter},
    relocation::{Relocation, Type as GenericType},
};

/// Normative psABI relocation identifiers.
pub const R_X86_64_NONE: u32 = 0;
pub const R_X86_64_RELATIVE: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    None,
    Relative,
    Other(GenericType),
}

impl Type {
    pub const fn from_generic(r#type: GenericType) -> Self {
        match r#type.raw() {
            R_X86_64_NONE => Self::None,
            R_X86_64_RELATIVE => Self::Relative,
            _ => Self::Other(r#type),
        }
    }

    pub const fn generic(self) -> GenericType {
        match self {
            Self::None => GenericType::from_raw(R_X86_64_NONE),
            Self::Relative => GenericType::from_raw(R_X86_64_RELATIVE),
            Self::Other(r#type) => r#type,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeError {
    WrongType { raw: u32 },
    SymbolIndexNotZero { symbol_index: u32 },
    MissingExplicitAddend,
    ValueOutOfRange { value: i128 },
    LoadTimeVirtualAddressOverflow { link_time_virtual_address: u64 },
    MemoryImage(memory_image::WriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelativeWrite {
    pub link_time_virtual_address: u64,
    pub load_time_virtual_address: u64,
    pub value: u64,
}

impl RelativeWrite {
    pub const fn representation(self) -> [u8; 8] {
        self.value.to_le_bytes()
    }

    pub fn apply(
        self,
        memory_image: &mut MemoryImageWriter<'_>,
    ) -> Result<(), RelativeError> {
        memory_image
            .write(self.load_time_virtual_address, &self.representation())
            .map_err(RelativeError::MemoryImage)
    }
}

/// Plan one `R_X86_64_RELATIVE` relocation.
///
/// AMD64 LP64 uses `Elf64_Rela`, so the addend is explicit. The psABI
/// calculation is `B + A`, where `B` is the object's base address and
/// `A` is `r_addend`. The relocation place is the load-time address
/// corresponding to `r_offset`.
pub fn relative_write(
    relocation: Relocation,
    base_address: BaseAddress,
) -> Result<RelativeWrite, RelativeError> {
    if relocation.r#type.raw() != R_X86_64_RELATIVE {
        return Err(RelativeError::WrongType {
            raw: relocation.r#type.raw(),
        });
    }

    if relocation.symbol_index != 0 {
        return Err(RelativeError::SymbolIndexNotZero {
            symbol_index: relocation.symbol_index,
        });
    }

    let addend = relocation
        .addend
        .ok_or(RelativeError::MissingExplicitAddend)?;

    let value = base_address.value() as i128 + addend as i128;
    let value = u64::try_from(value)
        .map_err(|_| RelativeError::ValueOutOfRange { value })?;

    let load_time_virtual_address = base_address
        .relocate_virtual_address(relocation.offset)
        .ok_or(RelativeError::LoadTimeVirtualAddressOverflow {
            link_time_virtual_address: relocation.offset,
        })?;

    Ok(RelativeWrite {
        link_time_virtual_address: relocation.offset,
        load_time_virtual_address,
        value,
    })
}

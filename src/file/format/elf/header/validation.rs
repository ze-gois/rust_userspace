//! gABI constraints over the ELF header.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    IdentificationVersionNotCurrent { version: u8 },
    IdentificationPaddingNotZero { index: usize, value: u8 },
    ObjectVersionNotCurrent { version: u32 },
    ReservedObjectType { raw: u16 },
    HeaderSizeTooSmall { size: u16, minimum: u16 },
    ProgramHeaderOffsetWithoutTable { offset: u64 },
    ProgramHeaderTableWithoutOffset { count: u16 },
    SectionHeaderOffsetWithoutTable { offset: u64 },
    SectionHeaderTableWithoutOffset { count: usize },
}

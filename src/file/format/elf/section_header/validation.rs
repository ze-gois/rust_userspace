//! gABI constraints over the ELF section-header table.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    AlignmentNotPowerOfTwo { index: usize },
    InformationLinkOutOfBounds { index: usize, target: usize },
    MergeOrStringsEntrySizeZero { index: usize },
    MergeOrStringsSizeNotEntryMultiple { index: usize },
    UndefinedSectionNameNotZero,
    UndefinedSectionTypeNotNull,
    UndefinedSectionFlagsNotZero,
    UndefinedSectionAddressNotZero,
    UndefinedSectionOffsetNotZero,
    UndefinedSectionInformationNotZero,
    UndefinedSectionAlignmentNotZero,
    UndefinedSectionEntrySizeNotZero,
    DirectSectionCountWithInitialSize { size: u64 },
    ExtendedSectionCountBelowReservedRange { count: usize },
    DirectSectionNameStringTableWithInitialLink { link: u32 },
    ExtendedSectionNameStringTableBelowReservedRange { index: usize },
    AddressMisaligned { index: usize },
    SectionOutsideFile { index: usize },
    SectionsOverlap { first: usize, second: usize },
    InvalidSectionNameStringTableType { index: usize },
    InvalidSectionName { index: usize, name_index: u32 },
}

//! gABI constraints over the ELF section-header table.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    AlignmentNotPowerOfTwo { index: usize },
    InformationLinkOutOfBounds { index: usize, target: usize },
    MergeOrStringsEntrySizeZero { index: usize },
    MergeOrStringsSizeNotEntryMultiple { index: usize },
}

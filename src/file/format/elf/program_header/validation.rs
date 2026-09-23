//! gABI constraints over the ELF program-header table.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    LoadFileImageLargerThanMemoryImage { index: usize },
    LoadAlignmentNotPowerOfTwo { index: usize },
    LoadAddressOffsetIncongruent { index: usize },
    LoadSegmentsNotOrderedByVirtualAddress { previous: usize, current: usize },
    MultipleInterpreters,
    InterpreterAfterLoad { index: usize },
    MultipleProgramHeaderTableImages,
    ProgramHeaderTableImageAfterLoad { index: usize },
    SharedLibrarySegment { index: usize },
    ReservedSegmentType { index: usize, raw: u32 },
    SegmentAlignmentNotPowerOfTwo { index: usize },
    SegmentAddressOffsetIncongruent { index: usize },
    SegmentFileImageOutsideFile { index: usize },
    InterpreterNotNullTerminated { index: usize },
    ThreadLocalStorageFileImageLargerThanTemplate { index: usize },
    ThreadLocalStorageFlagsNotReadOnly { index: usize, flags: u32 },
    ProgramHeaderTableImageMismatch { index: usize },
    ProgramHeaderTableNotLoaded { index: usize },
}

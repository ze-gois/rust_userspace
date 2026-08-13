use crate::file::format::elf::header::Header64 as ElfHeader64;
use crate::file::format::elf::segment::header::Header64 as ProgramHeader64;

#[derive(Debug, Clone, Copy)]
pub struct InterpreterPath {
    bytes: [u8; super::constants::MAX_INTERPRETER_PATH],
    len: usize,
}

impl InterpreterPath {
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.bytes[..self.len]).ok()
    }

    pub(super) fn from_parts(
        bytes: [u8; super::constants::MAX_INTERPRETER_PATH],
        len: usize,
    ) -> Self {
        Self { bytes, len }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoadedSegment {
    pub index: usize,
    pub address: u64,
    pub virtual_address: u64,
    pub file_offset: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub flags: u32,
    pub alignment: u64,
    pub map_start: u64,
    pub map_end: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct LoadedImage {
    pub entry: u64,
    pub base: u64,
    pub end: u64,
    pub direct_entry: bool,
    pub phdr: u64,
    pub phent: usize,
    pub phnum: usize,
    pub interpreter: Option<InterpreterPath>,
    pub dynamic: bool,
    pub segments: [Option<LoadedSegment>; 32],
    pub segment_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct PreparedExecution {
    pub image: LoadedImage,
    pub entry: u64,
    pub stack_pointer: crate::target::arch::PointerType,
}

#[derive(Clone, Copy)]
pub(super) struct SegmentPlan {
    pub(super) header: ProgramHeader64,
    pub(super) address: u64,
    pub(super) map_start: u64,
    pub(super) map_end: u64,
    pub(super) file_start: u64,
    pub(super) memory_end: u64,
}

#[derive(Clone, Copy)]
pub(super) struct ImagePlan {
    pub(super) segments: [Option<SegmentPlan>; 32],
    pub(super) segment_count: usize,
    pub(super) image_start: u64,
    pub(super) image_end: u64,
    pub(super) phdr: u64,
    pub(super) phent: usize,
    pub(super) phnum: usize,
    pub(super) interpreter: Option<InterpreterPath>,
    pub(super) dynamic: bool,
}

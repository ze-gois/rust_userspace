pub mod dtype;

pub mod header;
pub use header::Header32;
pub use header::Header64;

pub mod section;
pub mod segment;

pub mod transfer;

pub(super) const MAX_INTERPRETER_PATH: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct InterpreterPath {
    bytes: [u8; MAX_INTERPRETER_PATH],
    len: usize,
}

impl InterpreterPath {
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.bytes[..self.len]).ok()
    }

    pub(super) fn from_parts(bytes: [u8; MAX_INTERPRETER_PATH], len: usize) -> Self {
        Self { bytes, len }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoadedELF {
    pub entry: u64,
    pub base: u64,
    pub end: u64,
    pub direct_entry: bool,
    pub phdr: u64,
    pub phent: usize,
    pub phnum: usize,
    pub interpreter: Option<InterpreterPath>,
    pub dynamic: bool,
    pub segments: [Option<segment::LoadedSegment>; 32],
    pub segment_count: usize,
}

#[derive(Clone, Copy)]
pub struct LoadingPlan {
    pub segments: [Option<segment::types::SegmentLoadingPlan>; 32],
    pub segment_count: usize,
    pub image_start: u64,
    pub image_end: u64,
    pub phdr: u64,
    pub phent: usize,
    pub phnum: usize,
    pub interpreter: Option<InterpreterPath>,
    pub dynamic: bool,
}

pub fn execute_from_path(
    path: &str,
    stack_pointer: crate::target::arch::PointerType,
) -> core::result::Result<(), crate::file::format::elf::segment::Error> {
    let (is_elf, file_descriptor) =
        crate::file::format::elf::header::Identifier::is_file_path_magical(path);
    if file_descriptor >= 0 {
        let _ = crate::target::os::syscall::close(file_descriptor);
    }
    if !is_elf {
        return Err(crate::file::format::elf::segment::Error::InvalidHeader);
    }

    let prepared = match crate::file::format::elf::segment::prepare_execution(
        path,
        path.as_ptr(),
        stack_pointer,
    ) {
        Ok(prepared) => prepared,
        Err(error) => return Err(error),
    };

    let new_stack = crate::memory::Stack::from_pointer(crate::target::arch::Pointer(stack_pointer));
    new_stack.print();

    unsafe {
        crate::file::format::elf::transfer::jump_to_entry(prepared.entry, prepared.stack_pointer)
    }
}

pub mod result;
pub use result::{Error, Ok, Result};

// pub fn execute_from_path(path: &str) -> Result<!, Error> {
//     // let (header, file_descriptor) = header::Identifier::from_path(path)?;

// }

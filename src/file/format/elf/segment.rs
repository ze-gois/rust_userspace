pub mod header;

mod constants;
mod error;
mod io;
mod load;
mod mapping;
mod parse;
mod plan;

pub use error::Error;
pub use load::{load_inspect_path, load_path, load_static, load_static_path, prepare_execution};
// pub use types::{LoadedSegment, LoadingPlan, PreparedExecution};

use crate::file::format::elf::{LoadedELF, segment::header::Header64 as ProgramHeader64};

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
pub struct PreparedExecution {
    pub image: LoadedELF,
    pub entry: u64,
    pub stack_pointer: crate::target::arch::PointerType,
}

#[derive(Clone, Copy)]
pub struct LoadingPlan {
    //SegmentPlan
    pub header: ProgramHeader64,
    pub address: u64,
    pub map_start: u64,
    pub map_end: u64,
    pub file_start: u64,
    pub memory_end: u64,
}

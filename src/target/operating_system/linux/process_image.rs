//! Linux materialization of an ELF program image.
//!
//! The ELF layer describes the program image. This module realizes that image
//! in the current Linux process with `mmap(2)`, copies the segment contents,
//! and applies page protections with `mprotect(2)`.

use ample::r#type::Vec;

use crate::file::format::elf::{
    ObjectFile,
    base_address::BaseAddress,
    header::Type as ObjectType,
    program_header::Flags as ElfFlags,
    program_image,
};

use super::syscall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ProgramImage(program_image::Error),
    InvalidPageSize { size: usize },
    UnsupportedObjectType { object_type: ObjectType },
    AddressRangeOverflow,
    AddressUnsupported { address: u64 },
    MappingSizeUnsupported { size: u64 },
    MappingFailed,
    UnexpectedMappingAddress { expected: usize, actual: usize },
    SegmentAddressOutsideMapping { program_header_index: usize },
    SegmentSizeUnsupported { program_header_index: usize, size: u64 },
    ProtectionFailed { address: usize, length: usize },
}

#[derive(Debug)]
pub struct Mapping {
    address: *mut u8,
    length: usize,
    base_address: BaseAddress,
    page_size: usize,
}

impl Mapping {
    pub const fn address(&self) -> *mut u8 {
        self.address
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub const fn base_address(&self) -> BaseAddress {
        self.base_address
    }

    pub const fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn entry_address(&self, link_time_entry: u64) -> Option<*const u8> {
        let address = self
            .base_address
            .relocate_virtual_address(link_time_entry)?;
        usize::try_from(address).ok().map(|address| address as *const u8)
    }

    pub fn unmap(self) -> bool {
        syscall::munmap(self.address, self.length).is_ok()
    }
}

pub fn map(object_file: &ObjectFile<'_>, page_size: usize) -> Result<Mapping, Error> {
    if page_size == 0 || !page_size.is_power_of_two() {
        return Err(Error::InvalidPageSize { size: page_size });
    }

    let program_image = object_file
        .program_image()
        .map_err(Error::ProgramImage)?;

    let lowest = program_image
        .lowest_link_time_virtual_address()
        .ok_or(Error::ProgramImage(program_image::Error::NoLoadSegments))?;

    let highest = program_image
        .iter()
        .map(|segment| segment.link_time_end_virtual_address)
        .max()
        .ok_or(Error::ProgramImage(program_image::Error::NoLoadSegments))?;

    let page_size_u64 = page_size as u64;
    let mapping_link_time_start = align_down(lowest, page_size_u64);
    let mapping_link_time_end = align_up(highest, page_size_u64)
        .ok_or(Error::AddressRangeOverflow)?;
    let mapping_size_u64 = mapping_link_time_end
        .checked_sub(mapping_link_time_start)
        .ok_or(Error::AddressRangeOverflow)?;
    let mapping_size = usize::try_from(mapping_size_u64)
        .map_err(|_| Error::MappingSizeUnsupported {
            size: mapping_size_u64,
        })?;

    if mapping_size == 0 {
        return Err(Error::MappingSizeUnsupported { size: 0 });
    }

    let mut mapping_flags =
        syscall::mmap::Flag::PRIVATE | syscall::mmap::Flag::ANONYMOUS;

    let requested_address = match object_file.header.r#type {
        ObjectType::Executable => {
            mapping_flags |= syscall::mmap::Flag::FIXED_WITHOUT_REPLACEMENT;
            usize::try_from(mapping_link_time_start)
                .map_err(|_| Error::AddressUnsupported {
                    address: mapping_link_time_start,
                })? as *mut u8
        }
        ObjectType::SharedObject => core::ptr::null_mut(),
        object_type => {
            return Err(Error::UnsupportedObjectType { object_type });
        }
    };

    let initial_protection =
        (syscall::mmap::Protection::READ | syscall::mmap::Protection::WRITE).bits();

    let mapped_address = mmap_address(syscall::mmap(
        requested_address,
        mapping_size,
        initial_protection,
        mapping_flags.bits() as i32,
        -1,
        0,
    ))
    .ok_or(Error::MappingFailed)?;

    if matches!(object_file.header.r#type, ObjectType::Executable)
        && mapped_address != requested_address as usize
    {
        let _ = syscall::munmap(mapped_address as *mut u8, mapping_size);
        return Err(Error::UnexpectedMappingAddress {
            expected: requested_address as usize,
            actual: mapped_address,
        });
    }

    let base_address = match program_image.base_address(
        mapped_address as u64,
        page_size_u64,
    ) {
        Ok(base_address) => base_address,
        Err(error) => {
            let _ = syscall::munmap(mapped_address as *mut u8, mapping_size);
            return Err(Error::ProgramImage(error));
        }
    };

    if let Err(error) = copy_program_image(
        &program_image,
        mapped_address,
        mapping_size,
        base_address,
    ) {
        let _ = syscall::munmap(mapped_address as *mut u8, mapping_size);
        return Err(error);
    }

    if let Err(error) = protect_program_image(
        &program_image,
        mapped_address,
        mapping_size,
        base_address,
        page_size,
    ) {
        let _ = syscall::munmap(mapped_address as *mut u8, mapping_size);
        return Err(error);
    }

    Ok(Mapping {
        address: mapped_address as *mut u8,
        length: mapping_size,
        base_address,
        page_size,
    })
}

fn copy_program_image(
    program_image: &program_image::ProgramImage<'_>,
    mapping_address: usize,
    mapping_size: usize,
    base_address: BaseAddress,
) -> Result<(), Error> {
    let mapping_end = mapping_address
        .checked_add(mapping_size)
        .ok_or(Error::AddressRangeOverflow)?;

    for segment in program_image.iter() {
        let segment_address = segment
            .load_time_virtual_address(base_address)
            .map_err(Error::ProgramImage)?;
        let segment_address = usize::try_from(segment_address)
            .map_err(|_| Error::AddressUnsupported {
                address: segment_address,
            })?;

        let file_end = segment_address
            .checked_add(segment.file_image.len())
            .ok_or(Error::AddressRangeOverflow)?;
        if segment_address < mapping_address || file_end > mapping_end {
            return Err(Error::SegmentAddressOutsideMapping {
                program_header_index: segment.program_header_index,
            });
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                segment.file_image.as_ptr(),
                segment_address as *mut u8,
                segment.file_image.len(),
            );
        }

        if !segment.zero_fill.is_empty() {
            let zero_address = segment
                .zero_fill_load_time_virtual_address(base_address)
                .map_err(Error::ProgramImage)?;
            let zero_address = usize::try_from(zero_address)
                .map_err(|_| Error::AddressUnsupported {
                    address: zero_address,
                })?;
            let zero_size = usize::try_from(segment.zero_fill.size)
                .map_err(|_| Error::SegmentSizeUnsupported {
                    program_header_index: segment.program_header_index,
                    size: segment.zero_fill.size,
                })?;
            let zero_end = zero_address
                .checked_add(zero_size)
                .ok_or(Error::AddressRangeOverflow)?;

            if zero_address < mapping_address || zero_end > mapping_end {
                return Err(Error::SegmentAddressOutsideMapping {
                    program_header_index: segment.program_header_index,
                });
            }

            unsafe {
                core::ptr::write_bytes(zero_address as *mut u8, 0, zero_size);
            }
        }
    }

    Ok(())
}

fn protect_program_image(
    program_image: &program_image::ProgramImage<'_>,
    mapping_address: usize,
    mapping_size: usize,
    base_address: BaseAddress,
    page_size: usize,
) -> Result<(), Error> {
    let page_count = mapping_size / page_size;
    let mut protections = Vec::with_capacity(page_count);
    protections.resize(page_count, syscall::mmap::Protection::NONE.bits());

    for segment in program_image.iter() {
        let start = segment
            .load_time_virtual_address(base_address)
            .map_err(Error::ProgramImage)?;
        let end = segment
            .load_time_end_virtual_address(base_address)
            .map_err(Error::ProgramImage)?;

        let page_start = align_down(start, page_size as u64);
        let page_end = align_up(end, page_size as u64)
            .ok_or(Error::AddressRangeOverflow)?;

        let page_start = usize::try_from(page_start)
            .map_err(|_| Error::AddressUnsupported { address: page_start })?;
        let page_end = usize::try_from(page_end)
            .map_err(|_| Error::AddressUnsupported { address: page_end })?;

        let first_page = page_start
            .checked_sub(mapping_address)
            .ok_or(Error::SegmentAddressOutsideMapping {
                program_header_index: segment.program_header_index,
            })?
            / page_size;
        let final_page = page_end
            .checked_sub(mapping_address)
            .ok_or(Error::SegmentAddressOutsideMapping {
                program_header_index: segment.program_header_index,
            })?
            / page_size;

        if final_page > page_count {
            return Err(Error::SegmentAddressOutsideMapping {
                program_header_index: segment.program_header_index,
            });
        }

        let protection = linux_protection(segment.flags);
        for page in first_page..final_page {
            protections[page] |= protection;
        }
    }

    let mut run_start = 0usize;
    while run_start < page_count {
        let protection = protections[run_start];
        let mut run_end = run_start + 1;
        while run_end < page_count && protections[run_end] == protection {
            run_end += 1;
        }

        let address = mapping_address
            .checked_add(run_start * page_size)
            .ok_or(Error::AddressRangeOverflow)?;
        let length = (run_end - run_start) * page_size;

        if syscall::mprotect(address as *mut u8, length, protection).is_err() {
            return Err(Error::ProtectionFailed { address, length });
        }

        run_start = run_end;
    }

    Ok(())
}

fn linux_protection(flags: ElfFlags) -> i32 {
    let mut protection = syscall::mmap::Protection::NONE;
    if flags.readable() {
        protection |= syscall::mmap::Protection::READ;
    }
    if flags.writable() {
        protection |= syscall::mmap::Protection::WRITE;
    }
    if flags.executable() {
        protection |= syscall::mmap::Protection::EXECUTE;
    }
    protection.bits()
}

fn mmap_address(result: crate::Result) -> Option<usize> {
    match result {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::operating_system::Ok::Syscall(
                    crate::target::operating_system::syscall::Ok::MMap(
                        crate::target::operating_system::syscall::mmap::Ok::Default(address),
                    ),
                ),
            ),
        )) => Some(address),
        _ => None,
    }
}

const fn align_down(value: u64, alignment: u64) -> u64 {
    value & !(alignment - 1)
}

fn align_up(value: u64, alignment: u64) -> Option<u64> {
    value
        .checked_add(alignment - 1)
        .map(|value| align_down(value, alignment))
}

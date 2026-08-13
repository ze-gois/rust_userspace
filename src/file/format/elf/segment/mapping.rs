use crate::target::os::syscall;

use super::constants::{MAP_FAILED, PF_R, PF_W, PF_X};
use super::error::Error;
use super::types::{ImagePlan, LoadedSegment};

fn protections(flags: u32) -> i32 {
    let mut protection = syscall::mmap::Prot::None.to() as i32;
    if flags & PF_R != 0 {
        protection |= syscall::mmap::Prot::Read.to() as i32;
    }
    if flags & PF_W != 0 {
        protection |= syscall::mmap::Prot::Write.to() as i32;
    }
    if flags & PF_X != 0 {
        protection |= syscall::mmap::Prot::Exec.to() as i32;
    }
    protection
}

pub(super) fn map_image(
    file_descriptor: isize,
    plan: &ImagePlan,
    mapping: Option<u64>,
) -> Result<(), Error> {
    let image_length =
        usize::try_from(plan.image_end - plan.image_start).map_err(|_| Error::AddressOverflow)?;
    let mapping = match mapping {
        Some(address) => address,
        None => match syscall::mmap(
            plan.image_start as *mut u8,
            image_length,
            (syscall::mmap::Prot::Read.to() as i32) | (syscall::mmap::Prot::Write.to() as i32),
            (syscall::mmap::Flag::Private.to() as i32)
                | (syscall::mmap::Flag::Anonymous.to() as i32)
                | (syscall::mmap::Flag::FixedNoReplace.to() as i32),
            -1,
            0,
        ) {
            Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::MMap(syscall::mmap::Ok::Default(address)),
            )))) => address as u64,
            _ => return Err(Error::MappingFailed),
        },
    };

    if mapping == MAP_FAILED || mapping != plan.image_start {
        return Err(Error::MappingFailed);
    }

    for index in 0..plan.segment_count {
        let segment = plan.segments[index].ok_or(Error::InvalidProgramHeader)?;
        if segment.header.p_filesz.0 == 0 {
            continue;
        }
        if segment.file_start > i64::MAX as u64 {
            return Err(Error::AddressOverflow);
        }

        let mut copied = 0u64;
        while copied < segment.header.p_filesz.0 {
            let destination = (segment.address + copied) as *mut u8;
            let remaining = segment.header.p_filesz.0 - copied;
            let chunk = remaining.min(usize::MAX as u64) as usize;
            let read_count = match syscall::lseek(
                file_descriptor as i32,
                (segment.file_start + copied) as i64,
                syscall::lseek::Flag::SET.to(),
            ) {
                Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
                    crate::target::os::syscall::Ok::LSeek(syscall::lseek::Ok::Default(_)),
                )))) => match syscall::read(file_descriptor, destination, chunk) {
                    Ok(crate::Ok::Target(crate::target::Ok::Os(
                        crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Read(
                            syscall::read::Ok::Default(count),
                        )),
                    ))) => count,
                    _ => return Err(Error::FileReadFailed),
                },
                _ => return Err(Error::FileReadFailed),
            };
            if read_count == 0 || read_count > chunk {
                return Err(Error::FileReadFailed);
            }
            copied += read_count as u64;
        }
    }

    apply_permissions(plan)
}

fn apply_permissions(plan: &ImagePlan) -> Result<(), Error> {
    let mut boundaries = [0u64; 66];
    let mut boundary_count = 0usize;
    boundaries[boundary_count] = plan.image_start;
    boundary_count += 1;
    boundaries[boundary_count] = plan.image_end;
    boundary_count += 1;

    for index in 0..plan.segment_count {
        let segment = plan.segments[index].ok_or(Error::InvalidProgramHeader)?;
        boundaries[boundary_count] = segment.map_start;
        boundary_count += 1;
        boundaries[boundary_count] = segment.map_end;
        boundary_count += 1;
    }

    for index in 1..boundary_count {
        let value = boundaries[index];
        let mut position = index;
        while position > 0 && boundaries[position - 1] > value {
            boundaries[position] = boundaries[position - 1];
            position -= 1;
        }
        boundaries[position] = value;
    }

    let mut unique_count = 0usize;
    for index in 0..boundary_count {
        if unique_count == 0 || boundaries[index] != boundaries[unique_count - 1] {
            boundaries[unique_count] = boundaries[index];
            unique_count += 1;
        }
    }

    for index in 0..unique_count.saturating_sub(1) {
        let start = boundaries[index];
        let end = boundaries[index + 1];
        if start == end {
            continue;
        }

        let mut flags = 0u32;
        for segment_index in 0..plan.segment_count {
            let segment = plan.segments[segment_index].ok_or(Error::InvalidProgramHeader)?;
            if segment.map_start <= start && end <= segment.map_end {
                flags |= segment.header.p_flags.0;
            }
        }

        match syscall::mprotect(
            start as *mut u8,
            usize::try_from(end - start).map_err(|_| Error::AddressOverflow)?,
            protections(flags),
        ) {
            Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::MProtect(syscall::mprotect::Ok::Default(_)),
            )))) => {}
            _ => return Err(Error::ProtectionFailed),
        }
    }
    Ok(())
}

pub(super) fn segment_metadata(plan: &ImagePlan) -> [Option<LoadedSegment>; 32] {
    let mut segments = [None; 32];
    for index in 0..plan.segment_count {
        if let Some(segment) = plan.segments[index] {
            segments[index] = Some(LoadedSegment {
                index,
                address: segment.address,
                virtual_address: segment.header.p_vaddr.0,
                file_offset: segment.header.p_offset.0,
                file_size: segment.header.p_filesz.0,
                memory_size: segment.header.p_memsz.0,
                flags: segment.header.p_flags.0,
                alignment: segment.header.p_align.0,
                map_start: segment.map_start,
                map_end: segment.map_end,
            });
        }
    }
    segments
}

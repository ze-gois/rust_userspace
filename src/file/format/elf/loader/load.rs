use crate::file::format::elf::header::Header64;
use crate::target::os::syscall;

use super::constants::{ET_EXEC, MAP_FAILED};
use super::error::Error;
use super::io::file_size;
use super::mapping::{map_image, segment_metadata};
use super::parse::{read_header, validate_header};
use super::plan::{build_plan, entry_is_executable};
use super::types::LoadedImage;

fn load_file_descriptor(file_descriptor: isize) -> Result<LoadedImage, Error> {
    let (header, endianness) = read_header(file_descriptor)?;
    validate_header(header, endianness)?;
    let size = file_size(file_descriptor)?;
    let initial_plan = build_plan(size, file_descriptor, header, endianness, 0, false)?;

    let (plan, entry, base) = if header.e_type.0 == ET_EXEC {
        map_image(file_descriptor, &initial_plan, None)?;
        (initial_plan, header.e_entry.0, 0)
    } else {
        let length = usize::try_from(initial_plan.image_end - initial_plan.image_start)
            .map_err(|_| Error::AddressOverflow)?;
        let mapping = match syscall::mmap(
            core::ptr::null_mut(),
            length,
            (syscall::mmap::Prot::Read.to() as i32) | (syscall::mmap::Prot::Write.to() as i32),
            (syscall::mmap::Flag::Private.to() as i32)
                | (syscall::mmap::Flag::Anonymous.to() as i32),
            -1,
            0,
        ) {
            Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::MMap(syscall::mmap::Ok::Default(address)),
            )))) if address != MAP_FAILED as usize => address as u64,
            _ => return Err(Error::MappingFailed),
        };
        let base = mapping
            .checked_sub(initial_plan.image_start)
            .ok_or(Error::AddressOverflow)?;
        let relocated = build_plan(size, file_descriptor, header, endianness, base, false)?;
        map_image(file_descriptor, &relocated, Some(mapping))?;
        (
            relocated,
            header
                .e_entry
                .0
                .checked_add(base)
                .ok_or(Error::AddressOverflow)?,
            base,
        )
    };

    let direct_entry = header.e_type.0 == ET_EXEC
        && plan.interpreter.is_none()
        && !plan.dynamic
        && entry_is_executable(&plan, entry)?;

    Ok(LoadedImage {
        entry,
        base,
        end: plan.image_end,
        direct_entry,
        phdr: plan.phdr,
        phent: plan.phent,
        phnum: plan.phnum,
        interpreter: plan.interpreter,
        dynamic: plan.dynamic,
        segments: segment_metadata(&plan),
        segment_count: plan.segment_count,
    })
}

pub fn load_static(
    file_descriptor: isize,
    header: Header64,
    endianness: bool,
) -> Result<LoadedImage, Error> {
    validate_header(header, endianness)?;
    if header.e_type.0 != ET_EXEC {
        return Err(Error::UnsupportedType);
    }
    let size = file_size(file_descriptor)?;
    let plan = build_plan(size, file_descriptor, header, endianness, 0, true)?;
    map_image(file_descriptor, &plan, None)?;
    let entry = header.e_entry.0;
    if !entry_is_executable(&plan, entry)? {
        return Err(Error::EntryOutsideExecutableSegment);
    }
    Ok(LoadedImage {
        entry,
        base: 0,
        end: plan.image_end,
        direct_entry: true,
        phdr: plan.phdr,
        phent: plan.phent,
        phnum: plan.phnum,
        interpreter: None,
        dynamic: false,
        segments: segment_metadata(&plan),
        segment_count: plan.segment_count,
    })
}

pub fn load_path(path: &str) -> Result<LoadedImage, Error> {
    let file_descriptor = crate::file::open(path);
    if file_descriptor < 0 {
        return Err(Error::FileOpenFailed);
    }
    let result = load_file_descriptor(file_descriptor);
    let _ = syscall::close(file_descriptor);
    result
}

pub fn load_static_path(path: &str) -> Result<LoadedImage, Error> {
    let file_descriptor = crate::file::open(path);
    if file_descriptor < 0 {
        return Err(Error::FileOpenFailed);
    }
    let result = (|| {
        let (header, endianness) = read_header(file_descriptor)?;
        load_static(file_descriptor, header, endianness)
    })();
    let _ = syscall::close(file_descriptor);
    result
}

pub fn load_inspect_path(path: &str) -> Result<LoadedImage, Error> {
    load_path(path)
}

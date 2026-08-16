use crate::file::format::elf::header::Header64;
use crate::target::os::syscall;

use super::super::LoadedELF;
use super::PreparedExecution;
use super::constants::{ET_EXEC, MAP_FAILED};
use super::error::Error;
use super::mapping::{map_image, segment_metadata};
use super::parse::{read_header, validate_header};
use super::plan::{build_plan, entry_is_executable};

fn load_file_descriptor(file_descriptor: isize) -> Result<LoadedELF, Error> {
    let (header, endianness) = read_header(file_descriptor)?;
    validate_header(header, endianness)?;
    let size = super::io::file_size(file_descriptor)?;
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
        let base = match mapping.checked_sub(initial_plan.image_start) {
            Some(base) => base,
            None => {
                let _ = syscall::munmap(mapping as *mut u8, length);
                return Err(Error::AddressOverflow);
            }
        };
        let relocated = match build_plan(size, file_descriptor, header, endianness, base, false) {
            Ok(plan) => plan,
            Err(error) => {
                let _ = syscall::munmap(mapping as *mut u8, length);
                return Err(error);
            }
        };
        if let Err(error) = map_image(file_descriptor, &relocated, Some(mapping)) {
            let _ = syscall::munmap(mapping as *mut u8, length);
            return Err(error);
        }
        let entry = match header.e_entry.0.checked_add(base) {
            Some(entry) => entry,
            None => {
                let _ = syscall::munmap(mapping as *mut u8, length);
                return Err(Error::AddressOverflow);
            }
        };
        (relocated, entry, base)
    };

    let entry_is_executable = match entry_is_executable(&plan, entry) {
        Ok(value) => value,
        Err(error) => {
            let length = usize::try_from(plan.image_end - plan.image_start)
                .map_err(|_| Error::AddressOverflow)?;
            let _ = syscall::munmap(plan.image_start as *mut u8, length);
            return Err(error);
        }
    };
    if !entry_is_executable {
        let length = usize::try_from(plan.image_end - plan.image_start)
            .map_err(|_| Error::AddressOverflow)?;
        let _ = syscall::munmap(plan.image_start as *mut u8, length);
        return Err(Error::EntryOutsideExecutableSegment);
    }

    let direct_entry = plan.interpreter.is_none() && !plan.runtime_dynamic;

    Ok(LoadedELF {
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
) -> Result<LoadedELF, Error> {
    validate_header(header, endianness)?;
    if header.e_type.0 != ET_EXEC {
        return Err(Error::UnsupportedType);
    }
    let size = super::io::file_size(file_descriptor)?;
    let plan = build_plan(size, file_descriptor, header, endianness, 0, true)?;
    map_image(file_descriptor, &plan, None)?;
    let entry = header.e_entry.0;
    let entry_is_executable = match entry_is_executable(&plan, entry) {
        Ok(value) => value,
        Err(error) => {
            let length = usize::try_from(plan.image_end - plan.image_start)
                .map_err(|_| Error::AddressOverflow)?;
            let _ = syscall::munmap(plan.image_start as *mut u8, length);
            return Err(error);
        }
    };
    if !entry_is_executable {
        let length = usize::try_from(plan.image_end - plan.image_start)
            .map_err(|_| Error::AddressOverflow)?;
        let _ = syscall::munmap(plan.image_start as *mut u8, length);
        return Err(Error::EntryOutsideExecutableSegment);
    }
    Ok(LoadedELF {
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

pub fn load_path(path: &str) -> Result<LoadedELF, Error> {
    let file_descriptor = crate::file::open(path);
    if file_descriptor < 0 {
        return Err(Error::FileOpenFailed);
    }
    let result = load_file_descriptor(file_descriptor);
    let _ = syscall::close(file_descriptor);
    result
}

pub fn load_static_path(path: &str) -> Result<LoadedELF, Error> {
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

pub fn load_inspect_path(path: &str) -> Result<LoadedELF, Error> {
    load_path(path)
}

pub fn prepare_execution(
    path: &str,
    path_pointer: *const u8,
    initial_stack: crate::target::arch::PointerType,
) -> Result<PreparedExecution, Error> {
    let image = load_path(path)?;
    let (entry, interpreter_base) = match image.interpreter {
        Some(interpreter) => {
            let interpreter_path = interpreter.as_str().ok_or(Error::InvalidInterpreter)?;
            let interpreter_image = load_path(interpreter_path)?;
            if interpreter_image.interpreter.is_some() {
                return Err(Error::UnsupportedInterpreter);
            }
            (interpreter_image.entry, interpreter_image.base as usize)
        }
        None if image.direct_entry => (image.entry, 0),
        None => return Err(Error::InterpreterUnavailable),
    };

    let stack_pointer = crate::memory::Stack::build_execution_stack(
        initial_stack,
        path,
        path_pointer,
        image.entry,
        image.phdr,
        image.phent,
        image.phnum,
        interpreter_base,
    )
    .map_err(|_| Error::StackConstructionFailed)?;
    Ok(PreparedExecution {
        image,
        entry,
        stack_pointer,
    })
}

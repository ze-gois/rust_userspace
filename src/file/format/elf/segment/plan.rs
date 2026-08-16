use crate::file::format::elf::header::Header64;
use crate::file::format::elf::segment::header::Header64 as ProgramHeader64;
use crate::file::traits::Readable;

use crate::target::arch::page;

use super::LoadingPlan as SegmentLoadingPlan;
use super::constants::{PF_X, PT_DYNAMIC, PT_INTERP, PT_LOAD, PT_PHDR, PT_TLS};
use super::error::Error;
use super::io::{read_at, read_bytes};
use crate::file::format::elf::{InterpreterPath, LoadingPlan, MAX_INTERPRETER_PATH};

#[inline]
pub(super) fn checked_end(start: u64, size: u64) -> Option<u64> {
    start.checked_add(size)
}

fn read_program_header(
    file_descriptor: isize,
    offset: u64,
    endianness: bool,
) -> Result<ProgramHeader64, Error> {
    let bytes = read_at::<
        { <ProgramHeader64 as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE },
    >(file_descriptor, offset)?;
    Ok(ProgramHeader64::read_from_pointer(bytes.as_ptr(), 0, endianness).0)
}

const DT_NULL: u64 = 0;
const DT_NEEDED: u64 = 1;
const DT_PLTRELSZ: u64 = 2;

const DT_RELA: u64 = 7;
const DT_RELASZ: u64 = 8;
const DT_RELAENT: u64 = 9;
const DT_INIT: u64 = 12;
const DT_FINI: u64 = 13;
const DT_REL: u64 = 17;
const DT_RELSZ: u64 = 18;
const DT_RELENT: u64 = 19;
const DT_PLTREL: u64 = 20;

const DT_TEXTREL: u64 = 22;
const DT_JMPREL: u64 = 23;
const DT_INIT_ARRAY: u64 = 25;
const DT_FINI_ARRAY: u64 = 26;
const DT_INIT_ARRAYSZ: u64 = 27;
const DT_FINI_ARRAYSZ: u64 = 28;
const DT_PREINIT_ARRAY: u64 = 32;

fn dynamic_tag_requires_runtime_linker(tag: u64) -> bool {
    matches!(
        tag,
        DT_NEEDED
            | DT_PLTRELSZ
            | DT_RELA
            | DT_RELASZ
            | DT_RELAENT
            | DT_INIT
            | DT_FINI
            | DT_REL
            | DT_RELSZ
            | DT_RELENT
            | DT_PLTREL
            | DT_TEXTREL
            | DT_JMPREL
            | DT_INIT_ARRAY
            | DT_FINI_ARRAY
            | DT_INIT_ARRAYSZ
            | DT_FINI_ARRAYSZ
            | DT_PREINIT_ARRAY
    )
}

fn dynamic_requires_runtime_linker(
    file_descriptor: isize,
    file_size: u64,
    dynamic: Option<ProgramHeader64>,
    endianness: bool,
) -> Result<bool, Error> {
    let Some(dynamic) = dynamic else {
        return Ok(false);
    };

    if dynamic.p_filesz.0 == 0
        || dynamic.p_filesz.0 % 16 != 0
        || dynamic.p_filesz.0 > dynamic.p_memsz.0
    {
        return Err(Error::InvalidProgramHeader);
    }
    let file_end =
        checked_end(dynamic.p_offset.0, dynamic.p_filesz.0).ok_or(Error::AddressOverflow)?;
    if file_end > file_size {
        return Err(Error::InvalidProgramHeader);
    }

    let entry_count = dynamic.p_filesz.0 / 16;
    let mut requires_runtime = false;
    let mut found_null = false;
    for index in 0..entry_count {
        let offset = dynamic
            .p_offset
            .0
            .checked_add(index.checked_mul(16).ok_or(Error::AddressOverflow)?)
            .ok_or(Error::AddressOverflow)?;
        let bytes = read_at::<16>(file_descriptor, offset)?;
        let tag_bytes: [u8; 8] = bytes[..8]
            .try_into()
            .map_err(|_| Error::InvalidProgramHeader)?;
        let tag = if endianness {
            u64::from_le_bytes(tag_bytes)
        } else {
            u64::from_be_bytes(tag_bytes)
        };

        if tag == DT_NULL {
            found_null = true;
            break;
        }
        if dynamic_tag_requires_runtime_linker(tag) {
            requires_runtime = true;
        }
    }

    if !found_null {
        return Err(Error::InvalidProgramHeader);
    }
    Ok(requires_runtime)
}

fn read_interpreter(
    file_descriptor: isize,
    offset: u64,
    size: u64,
) -> Result<InterpreterPath, Error> {
    if size == 0 || size > MAX_INTERPRETER_PATH as u64 {
        return Err(Error::InvalidInterpreter);
    }

    let mut bytes = [0u8; MAX_INTERPRETER_PATH];
    let size = usize::try_from(size).map_err(|_| Error::AddressOverflow)?;
    read_bytes(file_descriptor, offset, &mut bytes[..size])?;

    let length = bytes[..size]
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(Error::InvalidInterpreter)?;
    if length == 0 {
        return Err(Error::InvalidInterpreter);
    }

    Ok(InterpreterPath::from_parts(bytes, length))
}

pub(super) fn build_plan(
    file_size: u64,
    file_descriptor: isize,
    header: Header64,
    endianness: bool,
    load_bias: u64,
    reject_runtime_features: bool,
) -> Result<LoadingPlan, Error> {
    let phoff = header.e_phoff.0;
    let phent = header.e_phentsize.0 as usize;
    let phnum = header.e_phnum.0 as usize;

    if phnum == 0
        || phnum > 32
        || phent
            != <ProgramHeader64 as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE
    {
        return Err(Error::InvalidProgramHeaderTable);
    }

    let table_size = (phent as u64)
        .checked_mul(phnum as u64)
        .ok_or(Error::AddressOverflow)?;
    let table_end = checked_end(phoff, table_size).ok_or(Error::InvalidProgramHeaderTable)?;
    if table_end > file_size {
        return Err(Error::InvalidProgramHeaderTable);
    }

    let mut segments = [None; 32];
    let mut segment_count = 0usize;
    let mut previous_load_address = 0u64;
    let mut image_start = u64::MAX;
    let mut image_end = 0u64;
    let mut phdr = None;
    let mut interpreter = None;
    let mut dynamic = false;
    let mut dynamic_header = None;
    let mut tls = false;

    for index in 0..phnum {
        let offset = phoff
            .checked_add((phent as u64) * index as u64)
            .ok_or(Error::AddressOverflow)?;
        let program_header = read_program_header(file_descriptor, offset, endianness)?;

        match program_header.p_type.0 {
            PT_INTERP => {
                if interpreter.is_some() {
                    return Err(Error::InvalidInterpreter);
                }
                interpreter = Some(read_interpreter(
                    file_descriptor,
                    program_header.p_offset.0,
                    program_header.p_filesz.0,
                )?);
            }
            PT_DYNAMIC => {
                if dynamic_header.is_some() {
                    return Err(Error::InvalidProgramHeader);
                }
                dynamic = true;
                dynamic_header = Some(program_header);
            }
            PT_TLS => tls = true,
            PT_PHDR => {
                phdr = Some(
                    program_header
                        .p_vaddr
                        .0
                        .checked_add(load_bias)
                        .ok_or(Error::AddressOverflow)?,
                );
            }
            PT_LOAD => {
                if segment_count > 0 && program_header.p_vaddr.0 < previous_load_address {
                    return Err(Error::InvalidProgramHeader);
                }
                previous_load_address = program_header.p_vaddr.0;

                let file_end = checked_end(program_header.p_offset.0, program_header.p_filesz.0)
                    .ok_or(Error::AddressOverflow)?;
                if file_end > file_size || program_header.p_filesz.0 > program_header.p_memsz.0 {
                    return Err(Error::InvalidProgramHeader);
                }
                let memory_end = checked_end(program_header.p_vaddr.0, program_header.p_memsz.0)
                    .ok_or(Error::AddressOverflow)?;
                if (program_header.p_vaddr.0 % page::SIZE as u64)
                    != (program_header.p_offset.0 % page::SIZE as u64)
                {
                    return Err(Error::InvalidProgramHeader);
                }
                if program_header.p_align.0 > 1
                    && (!program_header.p_align.0.is_power_of_two()
                        || (program_header.p_vaddr.0 % program_header.p_align.0)
                            != (program_header.p_offset.0 % program_header.p_align.0))
                {
                    return Err(Error::InvalidProgramHeader);
                }
                if program_header.p_memsz.0 == 0 {
                    continue;
                }

                let address = program_header
                    .p_vaddr
                    .0
                    .checked_add(load_bias)
                    .ok_or(Error::AddressOverflow)?;
                let relocated_end = memory_end
                    .checked_add(load_bias)
                    .ok_or(Error::AddressOverflow)?;
                let map_start = page::align_down(address);
                let map_end = page::align_up(relocated_end).ok_or(Error::AddressOverflow)?;
                if map_end < map_start || segment_count == 32 {
                    return Err(Error::AddressOverflow);
                }

                image_start = image_start.min(map_start);
                image_end = image_end.max(map_end);
                segments[segment_count] = Some(SegmentLoadingPlan {
                    header: program_header,
                    address,
                    map_start,
                    map_end,
                    file_start: program_header.p_offset.0,
                    memory_end: relocated_end,
                });
                segment_count += 1;
            }
            _ => {}
        }
    }

    if segment_count == 0 {
        return Err(Error::NoLoadableSegments);
    }

    let runtime_dynamic =
        dynamic_requires_runtime_linker(file_descriptor, file_size, dynamic_header, endianness)?;

    if reject_runtime_features {
        if interpreter.is_some() {
            return Err(Error::UnsupportedInterpreter);
        }
        if dynamic {
            return Err(Error::UnsupportedDynamicLinking);
        }
        if tls {
            return Err(Error::UnsupportedTls);
        }
    }

    let phdr = match phdr {
        Some(value) => value,
        None => {
            let table_end = phoff
                .checked_add(table_size)
                .ok_or(Error::AddressOverflow)?;
            let mut discovered = None;
            for index in 0..segment_count {
                let segment = segments[index].ok_or(Error::InvalidProgramHeader)?;
                let segment_file_end = segment
                    .header
                    .p_offset
                    .0
                    .checked_add(segment.header.p_filesz.0)
                    .ok_or(Error::AddressOverflow)?;
                if phoff >= segment.header.p_offset.0 && table_end <= segment_file_end {
                    discovered = Some(
                        segment
                            .address
                            .checked_add(phoff - segment.header.p_offset.0)
                            .ok_or(Error::AddressOverflow)?,
                    );
                    break;
                }
            }
            discovered.ok_or(Error::InvalidProgramHeaderTable)?
        }
    };

    Ok(LoadingPlan {
        segments,
        segment_count,
        image_start,
        image_end,
        phdr,
        phent,
        phnum,
        interpreter,
        dynamic,
        runtime_dynamic,
    })
}

pub(super) fn entry_is_executable(plan: &LoadingPlan, entry: u64) -> Result<bool, Error> {
    for index in 0..plan.segment_count {
        let segment = plan.segments[index].ok_or(Error::InvalidProgramHeader)?;
        if entry >= segment.address
            && entry < segment.memory_end
            && segment.header.p_flags.0 & PF_X != 0
        {
            return Ok(true);
        }
    }
    Ok(false)
}

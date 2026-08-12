use crate::file::format::elf::header::{Header64, Identifier};
use crate::file::format::elf::segment::header::Header64 as ProgramHeader64;
use crate::file::traits::Readable;
use crate::target::os::syscall;

const PAGE_SIZE: u64 = 0x1000;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const EM_X86_64: u16 = 62;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_PHDR: u32 = 6;
const PT_TLS: u32 = 7;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const MAP_FAILED: u64 = u64::MAX;
const MAX_INTERPRETER_PATH: usize = 256;
const STACK_SIZE: usize = 0x10000;
const PAGE_SIZE_USIZE: usize = PAGE_SIZE as usize;

const AT_PHDR: usize = 3;
const AT_PHENT: usize = 4;
const AT_PHNUM: usize = 5;
const AT_BASE: usize = 7;
const AT_ENTRY: usize = 9;
const AT_PLATFORM: usize = 15;
const AT_BASE_PLATFORM: usize = 24;
const AT_RANDOM: usize = 25;
const AT_EXECFN: usize = 31;
const AT_SYSINFO_EHDR: usize = 33;
const AT_NULL: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidHeader,
    UnsupportedClass,
    UnsupportedEndianness,
    UnsupportedType,
    UnsupportedMachine,
    InvalidProgramHeaderTable,
    InvalidProgramHeader,
    UnsupportedInterpreter,
    UnsupportedDynamicLinking,
    UnsupportedTls,
    NoLoadableSegments,
    EntryOutsideExecutableSegment,
    MappingFailed,
    ProtectionFailed,
    FileReadFailed,
    FileOpenFailed,
    FileMetadataFailed,
    AddressOverflow,
    InvalidInterpreter,
    InterpreterUnavailable,
    StackConstructionFailed,
}

#[derive(Debug, Clone, Copy)]
pub struct InterpreterPath {
    bytes: [u8; MAX_INTERPRETER_PATH],
    len: usize,
}

impl InterpreterPath {
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.bytes[..self.len]).ok()
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
struct SegmentPlan {
    header: ProgramHeader64,
    address: u64,
    map_start: u64,
    map_end: u64,
    file_start: u64,
    memory_end: u64,
}

#[derive(Clone, Copy)]
struct ImagePlan {
    segments: [Option<SegmentPlan>; 32],
    segment_count: usize,
    image_start: u64,
    image_end: u64,
    phdr: u64,
    phent: usize,
    phnum: usize,
    interpreter: Option<InterpreterPath>,
    dynamic: bool,
}

#[inline]
fn align_down(value: u64) -> u64 {
    value & !(PAGE_SIZE - 1)
}

#[inline]
fn align_up(value: u64) -> Option<u64> {
    value.checked_add(PAGE_SIZE - 1).map(align_down)
}

#[inline]
fn checked_end(start: u64, size: u64) -> Option<u64> {
    start.checked_add(size)
}

fn read_bytes(file_descriptor: isize, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
    if offset > i64::MAX as u64 {
        return Err(Error::AddressOverflow);
    }

    match syscall::lseek(
        file_descriptor as i32,
        offset as i64,
        syscall::lseek::Flag::SET.to(),
    ) {
        Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
            crate::target::os::syscall::Ok::LSeek(syscall::lseek::Ok::Default(_)),
        )))) => {}
        _ => return Err(Error::FileReadFailed),
    }

    let mut copied = 0usize;
    while copied < bytes.len() {
        let read_count = match syscall::read(
            file_descriptor,
            unsafe { bytes.as_mut_ptr().add(copied) },
            bytes.len() - copied,
        ) {
            Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::Read(syscall::read::Ok::Default(count)),
            )))) => count,
            _ => return Err(Error::FileReadFailed),
        };
        if read_count == 0 || read_count > bytes.len() - copied {
            return Err(Error::FileReadFailed);
        }
        copied += read_count;
    }
    Ok(())
}

fn read_at<const N: usize>(file_descriptor: isize, offset: u64) -> Result<[u8; N], Error> {
    let mut bytes = [0u8; N];
    read_bytes(file_descriptor, offset, &mut bytes)?;
    Ok(bytes)
}

fn file_size(file_descriptor: isize) -> Result<u64, Error> {
    let size = crate::file::information::from_fd(file_descriptor).st_size;
    if size < 0 {
        return Err(Error::FileMetadataFailed);
    }
    Ok(size as u64)
}

fn read_program_header(
    file_descriptor: isize,
    offset: u64,
    endianness: bool,
) -> Result<ProgramHeader64, Error> {
    let bytes = read_at::<{ core::mem::size_of::<ProgramHeader64>() }>(file_descriptor, offset)?;
    Ok(ProgramHeader64::read_from_pointer(bytes.as_ptr(), 0, endianness).0)
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
    read_bytes(
        file_descriptor,
        offset,
        &mut bytes[..usize::try_from(size).map_err(|_| Error::AddressOverflow)?],
    )?;

    let length = bytes[..usize::try_from(size).map_err(|_| Error::AddressOverflow)?]
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(Error::InvalidInterpreter)?;
    if length == 0 {
        return Err(Error::InvalidInterpreter);
    }

    Ok(InterpreterPath { bytes, len: length })
}

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

fn build_plan(
    file_size: u64,
    file_descriptor: isize,
    header: Header64,
    endianness: bool,
    load_bias: u64,
    reject_runtime_features: bool,
) -> Result<ImagePlan, Error> {
    let phoff = header.e_phoff.0;
    let phent = header.e_phentsize.0 as usize;
    let phnum = header.e_phnum.0 as usize;

    if phnum == 0 || phnum > 32 || phent != core::mem::size_of::<ProgramHeader64>() {
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
            PT_DYNAMIC => dynamic = true,
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
                if (program_header.p_vaddr.0 % PAGE_SIZE) != (program_header.p_offset.0 % PAGE_SIZE)
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
                let map_start = align_down(address);
                let map_end = align_up(relocated_end).ok_or(Error::AddressOverflow)?;
                if map_end < map_start || segment_count == 32 {
                    return Err(Error::AddressOverflow);
                }

                image_start = image_start.min(map_start);
                image_end = image_end.max(map_end);
                segments[segment_count] = Some(SegmentPlan {
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

    Ok(ImagePlan {
        segments,
        segment_count,
        image_start,
        image_end,
        phdr,
        phent,
        phnum,
        interpreter,
        dynamic,
    })
}

fn map_image(file_descriptor: isize, plan: &ImagePlan, mapping: Option<u64>) -> Result<(), Error> {
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

fn segment_metadata(plan: &ImagePlan) -> [Option<LoadedSegment>; 32] {
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

fn validate_header(header: Header64, endianness: bool) -> Result<(), Error> {
    if header.e_ident.class.0 != ELFCLASS64 {
        return Err(Error::UnsupportedClass);
    }
    if header.e_ident.data.0 != ELFDATA2LSB || !endianness {
        return Err(Error::UnsupportedEndianness);
    }
    if header.e_ident.version.0 != EV_CURRENT {
        return Err(Error::InvalidHeader);
    }
    if header.e_type.0 != ET_EXEC && header.e_type.0 != ET_DYN {
        return Err(Error::UnsupportedType);
    }
    if header.e_machine.0 != EM_X86_64 {
        return Err(Error::UnsupportedMachine);
    }
    if header.e_ehsize.0 as usize != core::mem::size_of::<Header64>() {
        return Err(Error::InvalidHeader);
    }
    Ok(())
}

fn read_header(file_descriptor: isize) -> Result<(Header64, bool), Error> {
    let identifier_bytes = read_at::<{ core::mem::size_of::<Identifier>() }>(file_descriptor, 0)?;
    let identifier = Identifier::read_from_pointer(identifier_bytes.as_ptr(), 0, true).0;
    if !identifier.is_magical() {
        return Err(Error::InvalidHeader);
    }
    let endianness = match identifier.data() {
        crate::file::format::elf::header::identifier::Data::DataLSB => true,
        _ => return Err(Error::UnsupportedEndianness),
    };
    let header_bytes = read_at::<{ core::mem::size_of::<Header64>() }>(file_descriptor, 0)?;
    let header = Header64::read_from_pointer(header_bytes.as_ptr(), 0, endianness).0;
    Ok((header, endianness))
}

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

fn entry_is_executable(plan: &ImagePlan, entry: u64) -> Result<bool, Error> {
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

unsafe fn count_null_terminated(pointer: *const usize, limit: usize) -> Option<usize> {
    for index in 0..limit {
        if unsafe { *pointer.add(index) } == 0 {
            return Some(index);
        }
    }
    None
}

unsafe fn c_string_size(pointer: *const u8, limit: usize) -> Option<usize> {
    if pointer.is_null() {
        return None;
    }

    for index in 0..limit {
        if unsafe { *pointer.add(index) } == 0 {
            return index.checked_add(1);
        }
    }
    None
}

unsafe fn copy_c_string(source: *const u8, destination: *mut u8, limit: usize) -> Option<usize> {
    let size = unsafe { c_string_size(source, limit) }?;
    unsafe {
        core::ptr::copy_nonoverlapping(source, destination, size);
    }
    Some(size)
}

fn update_auxiliary(
    key: usize,
    value: &mut usize,
    image: &LoadedImage,
    interpreter_base: usize,
    execfn: usize,
) {
    match key {
        AT_PHDR => *value = image.phdr as usize,
        AT_PHENT => *value = image.phent,
        AT_PHNUM => *value = image.phnum,
        AT_BASE => *value = interpreter_base,
        AT_ENTRY => *value = image.entry as usize,
        AT_EXECFN => *value = execfn,
        _ => {}
    }
}

fn build_initial_stack(
    initial_stack: crate::target::arch::PointerType,
    path: &str,
    _path_pointer: *const u8,
    image: &LoadedImage,
    interpreter_base: usize,
) -> Result<crate::target::arch::PointerType, Error> {
    let original = initial_stack as *const usize;
    let original_argc = unsafe { *original };
    if original_argc > 4096 {
        return Err(Error::StackConstructionFailed);
    }

    let old_argv = unsafe { original.add(1) };
    let old_envp = unsafe { old_argv.add(original_argc + 1) };
    let env_count =
        unsafe { count_null_terminated(old_envp, 4096) }.ok_or(Error::StackConstructionFailed)?;
    let old_auxv = unsafe { old_envp.add(env_count + 1) };
    let mut aux_count = 0usize;
    while aux_count < 1024 {
        let key = unsafe { *old_auxv.add(aux_count * 2) };
        aux_count += 1;
        if key == AT_NULL {
            break;
        }
    }
    if aux_count == 1024 {
        return Err(Error::StackConstructionFailed);
    }

    // The loader's argv[0] is not part of the target process. The target path
    // becomes argv[0], followed by the arguments originally passed after it.
    let new_argc = if original_argc >= 2 {
        original_argc - 1
    } else {
        1
    };

    let words = 1usize
        .checked_add(new_argc + 1)
        .and_then(|value| value.checked_add(env_count + 1))
        .and_then(|value| value.checked_add(aux_count.checked_mul(2)?))
        .ok_or(Error::StackConstructionFailed)?;
    let word_bytes = words
        .checked_mul(core::mem::size_of::<usize>())
        .ok_or(Error::StackConstructionFailed)?;

    // Calculate the independent data area before mapping anything. A bounded
    // scan prevents an unterminated source string from overrunning the old
    // process image while the new stack is being prepared.
    let mut data_bytes = path
        .as_bytes()
        .len()
        .checked_add(1)
        .ok_or(Error::StackConstructionFailed)?;

    for index in 1..new_argc {
        let source = unsafe { *old_argv.add(index + 1) as *const u8 };
        let size =
            unsafe { c_string_size(source, STACK_SIZE) }.ok_or(Error::StackConstructionFailed)?;
        data_bytes = data_bytes
            .checked_add(size)
            .ok_or(Error::StackConstructionFailed)?;
    }

    for index in 0..env_count {
        let source = unsafe { *old_envp.add(index) as *const u8 };
        let size =
            unsafe { c_string_size(source, STACK_SIZE) }.ok_or(Error::StackConstructionFailed)?;
        data_bytes = data_bytes
            .checked_add(size)
            .ok_or(Error::StackConstructionFailed)?;
    }

    for index in 0..aux_count {
        let key = unsafe { *old_auxv.add(index * 2) };
        let value = unsafe { *old_auxv.add(index * 2 + 1) };
        let extra = match key {
            AT_RANDOM if value != 0 => 16,
            AT_PLATFORM | AT_BASE_PLATFORM if value != 0 => unsafe {
                c_string_size(value as *const u8, STACK_SIZE)
                    .ok_or(Error::StackConstructionFailed)?
            },
            _ => 0,
        };
        data_bytes = data_bytes
            .checked_add(extra)
            .ok_or(Error::StackConstructionFailed)?;
    }

    let required_bytes = word_bytes
        .checked_add(15)
        .and_then(|value| value.checked_add(data_bytes))
        .ok_or(Error::StackConstructionFailed)?;
    if required_bytes > STACK_SIZE {
        return Err(Error::StackConstructionFailed);
    }

    // Keep one inaccessible page below the usable stack. This catches a
    // downward-growing stack crossing its lower boundary immediately.
    let mapping_length = STACK_SIZE
        .checked_add(PAGE_SIZE_USIZE)
        .ok_or(Error::StackConstructionFailed)?;
    let stack_address = match syscall::mmap(
        core::ptr::null_mut(),
        mapping_length,
        (syscall::mmap::Prot::Read.to() as i32) | (syscall::mmap::Prot::Write.to() as i32),
        (syscall::mmap::Flag::Private.to() as i32) | (syscall::mmap::Flag::Anonymous.to() as i32),
        -1,
        0,
    ) {
        Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
            crate::target::os::syscall::Ok::MMap(syscall::mmap::Ok::Default(address)),
        )))) if address != MAP_FAILED as usize => address,
        _ => return Err(Error::StackConstructionFailed),
    };

    let unmap_stack = || {
        let _ = syscall::munmap(stack_address as *mut u8, mapping_length);
    };

    match syscall::mprotect(
        stack_address as *mut u8,
        PAGE_SIZE_USIZE,
        syscall::mmap::Prot::None.to() as i32,
    ) {
        Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
            crate::target::os::syscall::Ok::MProtect(syscall::mprotect::Ok::Default(_)),
        )))) => {}
        _ => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    }

    let stack_top = match stack_address.checked_add(mapping_length) {
        Some(value) => value,
        None => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    };
    let stack_start = match stack_top.checked_sub(required_bytes) {
        Some(value) => value & !15usize,
        None => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    };
    let usable_start = match stack_address.checked_add(PAGE_SIZE_USIZE) {
        Some(value) => value,
        None => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    };
    if stack_start < usable_start {
        unmap_stack();
        return Err(Error::StackConstructionFailed);
    }

    let stack = stack_start as *mut usize;
    let data_start = match stack_start
        .checked_add(word_bytes)
        .and_then(|value| value.checked_add(15))
        .map(|value| value & !15usize)
    {
        Some(value) => value,
        None => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    };
    let data_end = match data_start.checked_add(data_bytes) {
        Some(value) if value <= stack_top => value,
        _ => {
            unmap_stack();
            return Err(Error::StackConstructionFailed);
        }
    };

    unsafe {
        *stack = new_argc;
        let argv = stack.add(1);
        let envp = argv.add(new_argc + 1);
        let auxv = envp.add(env_count + 1);
        let mut data_cursor = data_start as *mut u8;

        // argv[0] and AT_EXECFN share the copied target path.
        let target_path_destination = data_cursor;
        core::ptr::copy_nonoverlapping(
            path.as_bytes().as_ptr(),
            target_path_destination,
            path.as_bytes().len(),
        );
        *target_path_destination.add(path.as_bytes().len()) = 0;
        data_cursor = data_cursor.add(path.as_bytes().len() + 1);
        *argv = target_path_destination as usize;

        for index in 1..new_argc {
            let source = *old_argv.add(index + 1) as *const u8;
            let destination = data_cursor;
            let Some(size) = copy_c_string(source, destination, STACK_SIZE) else {
                unmap_stack();
                return Err(Error::StackConstructionFailed);
            };
            *argv.add(index) = destination as usize;
            data_cursor = data_cursor.add(size);
        }
        *argv.add(new_argc) = 0;

        for index in 0..env_count {
            let source = *old_envp.add(index) as *const u8;
            let destination = data_cursor;
            let Some(size) = copy_c_string(source, destination, STACK_SIZE) else {
                unmap_stack();
                return Err(Error::StackConstructionFailed);
            };
            *envp.add(index) = destination as usize;
            data_cursor = data_cursor.add(size);
        }
        *envp.add(env_count) = 0;

        for index in 0..aux_count {
            let key = *old_auxv.add(index * 2);
            let mut value = *old_auxv.add(index * 2 + 1);
            update_auxiliary(
                key,
                &mut value,
                image,
                interpreter_base,
                target_path_destination as usize,
            );

            match key {
                AT_RANDOM if value != 0 => {
                    let source = value as *const u8;
                    core::ptr::copy_nonoverlapping(source, data_cursor, 16);
                    value = data_cursor as usize;
                    data_cursor = data_cursor.add(16);
                }
                AT_PLATFORM | AT_BASE_PLATFORM if value != 0 => {
                    let source = value as *const u8;
                    let destination = data_cursor;
                    let Some(size) = copy_c_string(source, destination, STACK_SIZE) else {
                        unmap_stack();
                        return Err(Error::StackConstructionFailed);
                    };
                    value = destination as usize;
                    data_cursor = data_cursor.add(size);
                }
                // AT_SYSINFO_EHDR is the vDSO address and must remain a
                // pointer into the existing process mapping, not stack data.
                AT_SYSINFO_EHDR => {}
                _ => {}
            }

            *auxv.add(index * 2) = key;
            *auxv.add(index * 2 + 1) = value;
        }

        debug_assert_eq!(data_cursor as usize, data_end);
    }

    Ok(stack_start as crate::target::arch::PointerType)
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
            (interpreter_image.entry, interpreter_image.base as usize)
        }
        None if image.direct_entry => (image.entry, 0),
        None => return Err(Error::InterpreterUnavailable),
    };

    let stack_pointer =
        build_initial_stack(initial_stack, path, path_pointer, &image, interpreter_base)?;
    Ok(PreparedExecution {
        image,
        entry,
        stack_pointer,
    })
}

/// Transfer control using the Linux process-entry convention.
///
/// The target receives the newly prepared Linux initial stack in `%rsp`.
/// The dynamic interpreter uses that stack to relocate itself and the main
/// executable before transferring control to the program entry point.
pub unsafe fn jump_to_entry(entry: u64, initial_stack: crate::target::arch::PointerType) -> ! {
    unsafe {
        core::arch::asm!(
            "mov rsp, {stack}",
            "xor ebp, ebp",
            "jmp {entry}",
            stack = in(reg) initial_stack,
            entry = in(reg) entry,
            options(noreturn),
        );
    }
}

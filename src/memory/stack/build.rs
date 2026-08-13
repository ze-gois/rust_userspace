use crate::target::os::syscall;

use super::constants::{
    AT_BASE, AT_BASE_PLATFORM, AT_ENTRY, AT_EXECFN, AT_NULL, AT_PHDR, AT_PHENT, AT_PHNUM,
    AT_PLATFORM, AT_RANDOM, AT_SYSINFO_EHDR, STACK_SIZE,
};
use crate::file::format::elf::segment::types::LoadedImage;

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

pub fn build_initial_stack(
    initial_stack: crate::target::arch::PointerType,
    path: &str,
    _path_pointer: *const u8,
    image: &LoadedImage,
    interpreter_base: usize,
) -> Result<crate::target::arch::PointerType, crate::file::format::elf::segment::error::Error> {
    let original = initial_stack as *const usize;
    let original_argc = unsafe { *original };
    if original_argc > 4096 {
        return Err(crate::file::format::elf::segment::error::Error::AddressOverflow);
    }

    let old_argv = unsafe { original.add(1) };
    let old_envp = unsafe { old_argv.add(original_argc + 1) };
    let env_count = unsafe { count_null_terminated(old_envp, 4096) }
        .ok_or(crate::file::format::elf::segment::error::Error::AddressOverflow)?;
    let old_auxv = unsafe { old_envp.add(env_count + 1) };
    let mut aux_count = 0usize;
    let mut found_aux_null = false;
    while aux_count < 1024 {
        let key = unsafe { *old_auxv.add(aux_count * 2) };
        aux_count += 1;
        if key == AT_NULL {
            found_aux_null = true;
            break;
        }
    }
    if !found_aux_null {
        return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
    }

    let new_argc = if original_argc >= 2 {
        original_argc - 1
    } else {
        1
    };

    let words = 1usize
        .checked_add(new_argc + 1)
        .and_then(|value| value.checked_add(env_count + 1))
        .and_then(|value| value.checked_add(aux_count.checked_mul(2)?))
        .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
    let word_bytes = words
        .checked_mul(core::mem::size_of::<usize>())
        .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;

    let mut data_bytes = path
        .as_bytes()
        .len()
        .checked_add(1)
        .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;

    for index in 1..new_argc {
        let source = unsafe { *old_argv.add(index + 1) as *const u8 };
        let size = unsafe { c_string_size(source, STACK_SIZE) }
            .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
        data_bytes = data_bytes
            .checked_add(size)
            .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
    }

    for index in 0..env_count {
        let source = unsafe { *old_envp.add(index) as *const u8 };
        let size = unsafe { c_string_size(source, STACK_SIZE) }
            .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
        data_bytes = data_bytes
            .checked_add(size)
            .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
    }

    for index in 0..aux_count {
        let key = unsafe { *old_auxv.add(index * 2) };
        let value = unsafe { *old_auxv.add(index * 2 + 1) };
        let extra = match key {
            AT_RANDOM if value != 0 => 16,
            AT_PLATFORM | AT_BASE_PLATFORM if value != 0 => unsafe {
                c_string_size(value as *const u8, STACK_SIZE).ok_or(
                    crate::file::format::elf::segment::error::Error::StackConstructionFailed,
                )?
            },
            _ => 0,
        };
        data_bytes = data_bytes
            .checked_add(extra)
            .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
    }

    let required_bytes = word_bytes
        .checked_add(15)
        .and_then(|value| value.checked_add(data_bytes))
        .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
    if required_bytes > STACK_SIZE {
        return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
    }

    let mapping_length = STACK_SIZE
        .checked_add(crate::memory::page::SIZE)
        .ok_or(crate::file::format::elf::segment::error::Error::StackConstructionFailed)?;
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
        )))) if address != u64::MAX as usize => address,
        _ => return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed),
    };

    let unmap_stack = || {
        let _ = syscall::munmap(stack_address as *mut u8, mapping_length);
    };

    match syscall::mprotect(
        stack_address as *mut u8,
        crate::memory::page::SIZE,
        syscall::mmap::Prot::None.to() as i32,
    ) {
        Ok(crate::Ok::Target(crate::target::Ok::Os(crate::target::os::Ok::Syscall(
            crate::target::os::syscall::Ok::MProtect(syscall::mprotect::Ok::Default(_)),
        )))) => {}
        _ => {
            unmap_stack();
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    }

    let stack_top = match stack_address.checked_add(mapping_length) {
        Some(value) => value,
        None => {
            unmap_stack();
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    };
    let stack_start = match stack_top.checked_sub(required_bytes) {
        Some(value) => value & !15usize,
        None => {
            unmap_stack();
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    };
    let usable_start = match stack_address.checked_add(crate::memory::page::SIZE) {
        Some(value) => value,
        None => {
            unmap_stack();
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    };
    if stack_start < usable_start {
        unmap_stack();
        return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
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
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    };
    let data_end = match data_start.checked_add(data_bytes) {
        Some(value) if value <= stack_top => value,
        _ => {
            unmap_stack();
            return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
        }
    };

    unsafe {
        *stack = new_argc;
        let argv = stack.add(1);
        let envp = argv.add(new_argc + 1);
        let auxv = envp.add(env_count + 1);
        let mut data_cursor = data_start as *mut u8;

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
                return Err(
                    crate::file::format::elf::segment::error::Error::StackConstructionFailed,
                );
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
                return Err(
                    crate::file::format::elf::segment::error::Error::StackConstructionFailed,
                );
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
                        return Err(crate::file::format::elf::segment::error::Error::StackConstructionFailed);
                    };
                    value = destination as usize;
                    data_cursor = data_cursor.add(size);
                }
                AT_SYSINFO_EHDR => {}
                _ => {}
            }

            *auxv.add(index * 2) = key;
            *auxv.add(index * 2 + 1) = value;
        }

        debug_assert!((data_cursor as usize) <= data_end);
    }

    Ok(stack_start as crate::target::arch::PointerType)
}

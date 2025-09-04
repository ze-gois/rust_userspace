//! System call wrappers for image handling
//!
//! This module provides safe wrappers around system calls used by the image module,
//! primarily for memory allocation and file operations.

/// Memory-map a region of memory
///
/// # Safety
///
/// This function is unsafe because it directly calls the mmap syscall.
/// The caller must ensure proper usage of the allocated memory.
pub unsafe fn mmap_anonymous(size: usize) -> *mut u8 {
    match crate::target::os::syscall::mmap(
        core::ptr::null_mut(),
        size,
        (crate::target::os::syscall::mmap::Prot::Read
            | crate::target::os::syscall::mmap::Prot::Write) as i32,
        (crate::target::os::syscall::mmap::Flag::Anonymous
            | crate::target::os::syscall::mmap::Flag::Private) as i32,
        -1,
        0,
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                crate::target::os::syscall::mmap::Ok::Default(m),
            )),
        ))) => m as *mut u8,
        _ => core::ptr::null_mut(),
    }
}

/// Unmap a region of memory
///
/// # Safety
///
/// This function is unsafe because it directly calls the munmap syscall.
/// The caller must ensure the pointer and size are valid.
pub unsafe fn munmap(ptr: *mut u8, size: usize) -> bool {
    let result = crate::target::os::syscall::munmap(ptr as *mut u8, size);

    match result {
        core::result::Result::Ok(_) => true,
        _ => false,
    }
}

/// Open a file with specified flags
///
/// # Safety
///
/// This function is unsafe because it works with raw pointers.
pub unsafe fn open_file(path: *const u8, flags: i32) -> Option<i32> {
    let fd = crate::target::os::syscall::openat(
        crate::target::os::syscall::open::flags::AtFlag::FDCWD as isize,
        path,
        flags,
    );

    match fd {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::OPENAT(fd),
            )),
        ))) => Some(fd as i32),
        _ => None,
    }
}

/// Write data to a file descriptor
///
/// # Safety
///
/// This function is unsafe because it works with raw pointers.
pub unsafe fn write_file(fd: i32, data: *const u8, size: usize) -> bool {
    let result = crate::target::os::syscall::write(fd as isize, data as *const u8, size);

    match result {
        core::result::Result::Ok(_) => true,
        _ => false,
    }
}

/// Close a file descriptor
pub fn close_file(fd: i32) -> bool {
    let result = crate::target::os::syscall::close(fd);

    match result {
        core::result::Result::Ok(_) => true,
        _ => false,
    }
}

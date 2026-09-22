use ample::r#type::Vec;
use core::ffi::CStr;

use crate::target::os::syscall;

/// Read a regular file completely into an owned byte vector.
pub fn read(path: &CStr) -> Option<Vec<u8>> {
    let file_descriptor = match syscall::openat(
        syscall::open::AtFlag::FDCWD.to(),
        path.as_ptr().cast(),
        syscall::open::Flag::RDONLY.to(),
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::OPENAT(file_descriptor),
            )),
        ))) => file_descriptor as isize,
        _ => return None,
    };

    let length = match syscall::lseek(
        file_descriptor as i32,
        0,
        syscall::lseek::Whence::END.raw(),
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::LSeek(
                crate::target::os::syscall::lseek::Ok::Default(length),
            )),
        ))) => length,
        _ => {
            let _ = syscall::close(file_descriptor);
            return None;
        }
    };

    if syscall::lseek(
        file_descriptor as i32,
        0,
        syscall::lseek::Whence::SET.raw(),
    )
    .is_err()
    {
        let _ = syscall::close(file_descriptor);
        return None;
    }

    let mut bytes = Vec::with_capacity(length);
    bytes.resize(length, 0);

    let mut read = 0usize;
    while read < length {
        let count = match syscall::read(
            file_descriptor,
            bytes[read..].as_mut_ptr(),
            length - read,
        ) {
            core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Read(
                    crate::target::os::syscall::read::Ok::Default(count),
                )),
            ))) => count,
            _ => {
                let _ = syscall::close(file_descriptor);
                return None;
            }
        };

        if count == 0 {
            break;
        }

        read += count;
    }

    let _ = syscall::close(file_descriptor);
    bytes.truncate(read);
    Some(bytes)
}

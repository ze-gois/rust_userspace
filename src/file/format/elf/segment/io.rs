use crate::target::os::syscall;

use super::error::Error;

pub fn read_bytes(file_descriptor: isize, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
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

pub fn read_at<const N: usize>(file_descriptor: isize, offset: u64) -> Result<[u8; N], Error> {
    let mut bytes = [0u8; N];
    read_bytes(file_descriptor, offset, &mut bytes)?;
    Ok(bytes)
}

pub fn file_size(file_descriptor: isize) -> Result<u64, Error> {
    let size = crate::file::information::from_fd(file_descriptor).st_size;
    if size < 0 {
        return Err(Error::FileMetadataFailed);
    }
    Ok(size as u64)
}

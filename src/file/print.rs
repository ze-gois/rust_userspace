use crate::target::operating_system::syscall;

const PATH_CAPACITY: usize = 4096;
const BUFFER_CAPACITY: usize = 4096;

/// Print a file to standard output.
///
/// The pathname is converted locally to the NUL-terminated representation
/// required by Linux. The file is streamed in fixed-size chunks so printing
/// does not require allocation or the former file-loading machinery.
pub fn print(file_path: &str) {
    let bytes = file_path.as_bytes();
    if bytes.len() >= PATH_CAPACITY {
        return;
    }

    let mut path = [0u8; PATH_CAPACITY];
    path[..bytes.len()].copy_from_slice(bytes);

    let file_descriptor = match syscall::openat(
        syscall::open::AtFlag::FDCWD.to(),
        path.as_ptr(),
        syscall::open::Flag::RDONLY.to(),
        0,
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::Open(
                crate::target::operating_system::syscall::open::Ok::OPENAT(file_descriptor),
            )),
        ))) => file_descriptor as isize,
        _ => return,
    };

    let mut buffer = [0u8; BUFFER_CAPACITY];

    'printing: loop {
        let read_length = match syscall::read(
            file_descriptor,
            buffer.as_mut_ptr(),
            buffer.len(),
        ) {
            core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::Read(
                    crate::target::operating_system::syscall::read::Ok::Default(read_length),
                )),
            ))) => read_length,
            _ => break,
        };

        if read_length == 0 {
            break;
        }

        let mut written = 0usize;
        while written < read_length {
            let write_length = match syscall::write(
                1,
                buffer[written..read_length].as_ptr(),
                read_length - written,
            ) {
                core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
                    crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::Write(
                        crate::target::operating_system::syscall::write::Ok::Default(write_length),
                    )),
                ))) => write_length,
                _ => break 'printing,
            };

            if write_length == 0 {
                break 'printing;
            }

            written += write_length;
        }
    }

    let _ = syscall::close(file_descriptor);
}

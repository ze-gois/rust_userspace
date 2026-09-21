use crate::target::architecture::Architecture;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::READ;

pub fn read(file_descriptor: isize, byte_buffer: *const u8, byte_length: usize) -> crate::Result {
    let arch_result = Architecture::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    );

    handle_result(arch_result)
}

pub mod ok {
    ample::result!( Ok; "Read Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Read Error"; usize; [
        [1  ; ERROR  ; Default           ; usize ; "Error"  ; "Something wicked this way comes"],
        [4  ; EINTR  ; Interrupted       ; usize ; "EINTR"  ; "System call was interrupted"],
        [5  ; EIO    ; IOError           ; usize ; "EIO"    ; "Input/output error"],
        [9  ; EBADF  ; BadFileDescriptor ; usize ; "EBADF"  ; "Bad file descriptor"],
        [14 ; EFAULT ; InvalidBuffer     ; usize ; "EFAULT" ; "Invalid buffer pointer"],
        [22 ; EINVAL ; InvalidCount      ; usize ; "EINVAL" ; "Invalid count"],
        [21 ; EISDIR ; IsDirectory       ; usize ; "EISDIR" ; "Is a directory"],
        [13 ; EACCES ; NotReadable       ; usize ; "EACCES" ; "File not open for reading"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: crate::Result) -> crate::Result {
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall3(
                    crate::target::architecture::x86::bit64::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Read(
                crate::target::system::operating::linux::syscall::read::Ok::Default(m),
            )),
        ))),
        crate::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::Syscall(
                crate::target::architecture::x86::bit64::syscall::Error::Syscall3(
                    crate::target::architecture::x86::bit64::syscall::syscall3::Error::Default(raw),
                ),
            ),
        ))) => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Read(
                        Error::Default(raw),
                    ),
                ),
            ),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Read(
                        Error::Default(usize::MAX),
                    ),
                ),
            ),
        )),
    }
}

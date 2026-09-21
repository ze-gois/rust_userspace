use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::READ;

/// # Safety
///
/// `byte_buffer` must designate writable storage for at least `byte_count`
/// bytes.
pub unsafe fn read(
    file_descriptor: isize,
    byte_buffer: *mut u8,
    byte_count: usize,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_descriptor as usize,
            byte_buffer as usize,
            byte_count,
        )
    };

    handle_result(raw_return)
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

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Read(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Read(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}

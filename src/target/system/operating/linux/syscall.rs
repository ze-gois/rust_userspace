pub mod result;
pub use result::*;

pub mod number;

pub const MAXIMUM_ERROR_NUMBER: usize = 4095;

/// Linux's traditional `MAX_ERRNO` name.
pub const MAX_ERRNO: usize = MAXIMUM_ERROR_NUMBER;

/// Raw return value from a Linux system call.
///
/// The raw register value is preserved even when it encodes an errno.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorNumber(usize);

/// Conventional community alias for a Linux error number.
pub type Errno = ErrorNumber;

impl ErrorNumber {
    pub const fn from_number(number: usize) -> Self {
        Self(number)
    }

    pub const fn number(self) -> usize {
        self.0
    }
}

ample::r#struct!(
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct Success {
        raw: usize
    }
);

impl Success {
    pub const fn raw(self) -> usize {
        self.raw
    }
}

ample::r#struct!(
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct Failure {
        raw: usize
    }
);

impl Failure {
    pub const fn raw(self) -> usize {
        self.raw
    }

    pub const fn error_number(self) -> ErrorNumber {
        ErrorNumber::from_number((-(self.raw as isize)) as usize)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Return(usize);

impl Return {
    pub const fn new(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }

    pub const fn error_number(self) -> Option<ErrorNumber> {
        let signed = self.0 as isize;

        if signed < 0 && signed >= -(MAXIMUM_ERROR_NUMBER as isize) {
            Some(ErrorNumber::from_number((-signed) as usize))
        } else {
            None
        }
    }

    pub const fn is_error(self) -> bool {
        self.error_number().is_some()
    }

    pub const fn classify(self) -> core::result::Result<Success, Failure> {
        if self.is_error() {
            core::result::Result::Err(Failure { raw: self.0 })
        } else {
            core::result::Result::Ok(Success { raw: self.0 })
        }
    }
}

macro_rules! syscalls {
    (
        $(
            $module:ident => $variant:ident
        ),* $(,)?
    ) => {
        $(
            pub mod $module;
            pub use $module::$module;
        )*

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Syscall {
            $(
                $variant,
            )*
            Unknown(usize),
        }

        impl Syscall {
            pub const fn number(self) -> usize {
                match self {
                    $(
                        Self::$variant => $module::NUMBER,
                    )*
                    Self::Unknown(number) => number,
                }
            }

            pub const fn from_number(number: usize) -> Self {
                match number {
                    $(
                        $module::NUMBER => Self::$variant,
                    )*
                    _ => Self::Unknown(number),
                }
            }

            pub const fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($module),
                    )*
                    Self::Unknown(_) => "unknown",
                }
            }
        }

        impl From<Syscall> for usize {
            fn from(syscall: Syscall) -> Self {
                syscall.number()
            }
        }

        impl From<usize> for Syscall {
            fn from(number: usize) -> Self {
                Self::from_number(number)
            }
        }
    };
}

syscalls!(
    read => Read,
    write => Write,
    open => Open,
    close => Close,
    fstat => Fstat,
    lseek => Lseek,
    mmap => Mmap,
    mprotect => Mprotect,
    munmap => Munmap,
    fork => Fork,
    execve => Execve,
    exit => Exit,
    openat => Openat,
    getrandom => Getrandom,
);

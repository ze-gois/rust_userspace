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
pub struct Return(usize);

impl Return {
    pub const fn new(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }

    pub const fn error_number(self) -> Option<usize> {
        let signed = self.0 as isize;

        if signed < 0 && signed >= -(MAXIMUM_ERROR_NUMBER as isize) {
            Some((-signed) as usize)
        } else {
            None
        }
    }

    pub const fn is_error(self) -> bool {
        self.error_number().is_some()
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

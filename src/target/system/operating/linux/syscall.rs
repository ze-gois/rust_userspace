pub mod result;
pub use result::*;

pub mod number;

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

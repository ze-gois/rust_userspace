#[macro_export]
macro_rules! publish_syscalls {
    (
        $(
            [
                $syscall_signature:ident;
                $syscall_number:expr;
                $syscall_ident:ident;
                $($syscall_path:tt)::+;
                $syscall_label:expr
            ]
        ),
        *) => {
        $(pub mod $($syscall_path)::+;)*
        $(pub use $($syscall_path)::+::$($syscall_path)::+;)*

        // $(
        //     pub mod $($syscall_path)::+ {
        //         $(pub use super::$($syscall_path)::+::$($syscall_path)::+;)*
        //     }
        // )*

        pub mod numbers {
            $(pub const $syscall_ident : usize = $syscall_number;)*
        }

        pub mod labels {
            $(pub const $syscall_ident : &str = $syscall_label;)*
        }

        pub mod signatures {
            $(pub type $syscall_ident = crate::target::arch::traits::callable::$syscall_signature;)*
        }

        #[repr(usize)]
        pub enum Syscall {
            $(
                $syscall_ident = $syscall_number,
            )*
            TODO = usize::MAX,
        }

        impl Syscall {
            pub fn to_no(&self) -> usize {
                match self {
                    $(Syscall::$syscall_ident => $syscall_number,)*
                    Syscall::TODO => <usize>::MAX,
                }
            }

            pub fn from_no(n: usize) -> Syscall {
                match n {
                    $($syscall_number => Syscall::$syscall_ident,)*
                    _ => Syscall::TODO,
                }
            }
        }

        impl Into<usize> for Syscall {
            fn into(self) -> usize {
                self.to_no()
            }
        }
    };
}

pub use publish_syscalls;

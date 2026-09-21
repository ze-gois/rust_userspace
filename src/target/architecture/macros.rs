#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name($($arg: $type,)*) -> $crate::Result {
            $crate::target::architecture::Architecture::$syscall($($arg),*)
        }
    }
}
pub use wrap_syscall;

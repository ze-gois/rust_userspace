#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name($($arg: $type,)*) -> $crate::Result {
            let return_value = $crate::target::architecture::Arch::$syscall($($arg),*);
            $crate::info!(X "\tWrapped syscall");
            $($crate::info!(X "\t{:?},",$arg);)*
            $crate::info!(X "\t{:?}",return_value);

            return_value
        }
    }
}
pub use wrap_syscall;

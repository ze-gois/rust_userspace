#[macro_export]
macro_rules! file_format_elf_dtype_define {
    ($(#[$meta:meta])* $vis:vis $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis struct $name(pub $inner);

        impl $crate::file::format::elf::dtype::Trait for $name {
            type Inner = $inner;
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}",self.0)?;
                Ok(())
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0x{:#x}",self.0)?;
                Ok(())
            }
        }

        $crate::trait_implement_partial_eq!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        $crate::trait_implement_from!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
    }
}
pub use file_format_elf_dtype_define;

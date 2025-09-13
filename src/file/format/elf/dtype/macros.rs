#[macro_export]
macro_rules! file_format_elf_dtype_class {
    ($class:ident, $class_ident:ident, $(#[$meta:meta])* [$([$vis:vis $name:ident, $inner:ty]),* $(,)?]) => {
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

        ample::trait_implement_primitive!(false, $($name),*);
        ample::trait_implement_from!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        ample::trait_implement_partial_eq!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

        $(
            impl $name {
                pub fn read(fd: isize, endianness: bool) -> core::result::Result<$name,$crate::Error>
                where
                    [u8; <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE];

                    match $crate::target::os::syscall::read(fd, bytes.as_mut_ptr(), <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE) {
                        core::result::Result::Ok($crate::Ok::Target($crate::target::Ok::Os($crate::target::os::Ok::Syscall($crate::target::os::syscall::Ok::Write($crate::target::os::syscall::write::Ok::Default(value)))))) => {
                            if value != <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE {
                                return core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(value))))))));
                            }

                            let value = $name(match endianness {
                                true => <$name as ample::traits::Bytes<$crate::Origin>>::from_bytes(bytes,true).into(),
                                false => <$name as ample::traits::Bytes<$crate::Origin>>::from_bytes(bytes,false).into(),
                            });

                            // core::result::Result::Ok($crate::Ok::File($crate::file::Ok::Format($crate::file::format::Ok::Elf($crate::file::format::elf::Ok::DType($crate::file::format::elf::dtype::Ok::$class_ident($crate::file::format::elf::dtype::$class::Ok::$name($name(value.0))))))))
                            core::result::Result::Ok($name(value.0))
                        }
                        _ => core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(0))))))))
                    }
                }

                pub fn read_to_crate(fd: isize, endianness: bool) -> $crate::Result
                where
                    [u8; <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE];

                    match $crate::target::os::syscall::read(fd, bytes.as_mut_ptr(), <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE) {
                        core::result::Result::Ok($crate::Ok::Target($crate::target::Ok::Os($crate::target::os::Ok::Syscall($crate::target::os::syscall::Ok::Write($crate::target::os::syscall::write::Ok::Default(value)))))) => {
                            if value != <$name as ample::traits::Bytes<$crate::Origin>>::BYTES_SIZE {
                                return core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(value))))))));
                            }

                            let value = $name(match endianness {
                                true => <$name as ample::traits::Bytes<$crate::Origin>>::from_bytes(bytes,true).into(),
                                false => <$name as ample::traits::Bytes<$crate::Origin>>::from_bytes(bytes,false).into(),
                            });

                            core::result::Result::Ok($crate::Ok::File($crate::file::Ok::Format($crate::file::format::Ok::Elf($crate::file::format::elf::Ok::DType($crate::file::format::elf::dtype::Ok::$class_ident($crate::file::format::elf::dtype::$class::Ok::$name($name(value.0))))))))
                        }
                        _ => core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(0))))))))
                    }
                }
            }
        )*
    };
}

pub use file_format_elf_dtype_class;

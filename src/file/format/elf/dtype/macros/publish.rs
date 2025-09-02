#[macro_export]
macro_rules! publish_types {
    ($class:ident, $class_ident:ident, [$([$ident_vis:vis $ident:ident, $ident_type:ty]),* $(,)?]) => {
        $(
            $crate::file_format_elf_dtype_define!($ident_vis $ident, $ident_type);

        )*

        ample::trait_implement_primitive!(false, $($ident),*);
        ample::trait_implement_bytes!($($ident),*);
        $(
            impl $ident {
                pub fn read(fd: isize, endianness: bool) -> core::result::Result<$ident,$crate::Error>
                where
                    [u8; <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE];

                    match $crate::target::os::syscall::read(fd, bytes.as_mut_ptr(), <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE) {
                        core::result::Result::Ok($crate::Ok::Target($crate::target::Ok::Os($crate::target::os::Ok::Syscall($crate::target::os::syscall::Ok::Write($crate::target::os::syscall::write::Ok::Default(value)))))) => {
                            if value != <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE {
                                return core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(value))))))));
                            }

                            let value = $ident(match endianness {
                                true => <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(bytes,true).into(),
                                false => <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(bytes,false).into(),
                            });

                            // core::result::Result::Ok($crate::Ok::File($crate::file::Ok::Format($crate::file::format::Ok::Elf($crate::file::format::elf::Ok::DType($crate::file::format::elf::dtype::Ok::$class_ident($crate::file::format::elf::dtype::$class::Ok::$ident($ident(value.0))))))))
                            core::result::Result::Ok($ident(value.0))
                        }
                        _ => core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(0))))))))
                    }
                }

                pub fn read_to_crate(fd: isize, endianness: bool) -> $crate::Result
                where
                    [u8; <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE]:,
                {
                    let mut bytes = [0u8; <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE];

                    match $crate::target::os::syscall::read(fd, bytes.as_mut_ptr(), <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE) {
                        core::result::Result::Ok($crate::Ok::Target($crate::target::Ok::Os($crate::target::os::Ok::Syscall($crate::target::os::syscall::Ok::Write($crate::target::os::syscall::write::Ok::Default(value)))))) => {
                            if value != <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE {
                                return core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(value))))))));
                            }

                            let value = $ident(match endianness {
                                true => <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(bytes,true).into(),
                                false => <$ident as ample::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(bytes,false).into(),
                            });

                            core::result::Result::Ok($crate::Ok::File($crate::file::Ok::Format($crate::file::format::Ok::Elf($crate::file::format::elf::Ok::DType($crate::file::format::elf::dtype::Ok::$class_ident($crate::file::format::elf::dtype::$class::Ok::$ident($ident(value.0))))))))
                        }
                        _ => core::result::Result::Err($crate::Error::File($crate::file::Error::Format($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::Null(Null(0))))))))
                    }
                }
            }
        )*
    };
}

pub use publish_types;

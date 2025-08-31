#[macro_export]
macro_rules! file_format_elf_dtype_impl {
    ($class:ident, $class_ident:ident, $($dtype:ident),*) => {
        $crate::trait_implement_primitive!(false, $($dtype),*);
        $crate::trait_implement_bytes!($($dtype),*);
        $(
            impl $dtype {
                pub fn read(fd: isize, endianness: bool) -> $crate::Result {
                    const INNER_SIZE: usize = <$dtype as $crate::traits::Bytes<$crate::Origin, $crate::Origin>>::BYTES_SIZE;
                    let mut bytes = [0u8; INNER_SIZE];
                    match $crate::target::os::syscall::read(fd, bytes.as_mut_ptr(), INNER_SIZE) {
                        core::result::Result::Ok($crate::Ok::Target($crate::target::Ok::Os($crate::target::os::Ok::Syscall($crate::target::os::syscall::Ok::Write($crate::target::os::syscall::write::Ok::Default(value)))))) => {
                            if value != INNER_SIZE {
                                return core::result::Result::Err($crate::Error::File($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::$dtype(value))))));
                            }

                            let value  = match endianness {
                                true => <$dtype as $crate::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(&bytes,true),
                                false => <$dtype as $crate::traits::Bytes<$crate::Origin, $crate::Origin>>::from_bytes(&bytes,false),
                            };

                            core::result::Result::Ok($crate::Ok::File($crate::file::Ok::Format($crate::file::format::Ok::Elf($crate::file::format::elf::Ok::DType($crate::file::format::elf::dtype::Ok::$class_ident($crate::file::format::elf::dtype::$class::Ok::$dtype(value)))))))
                        }
                        _ => core::result::Result::Err($crate::Error::File($crate::file::format::Error::Elf($crate::file::format::elf::Error::DType($crate::file::format::elf::dtype::Error::$class_ident($crate::file::format::elf::dtype::$class::Error::$dtype(value))))))
                    }
                }
            }
                // pub fn to_bytes(
                //     &self,
                //     endianness: $crate::dtype::Endianness,
                // ) -> [u8; Self::SIZE_BYTES] {
                //     match endianness {
                //         Endianness::TODO => {
                //             $crate::info!("Endianness is TODO");
                //             panic!("TODO");
                //         }
                //         Endianness::None => {
                //             $crate::info!("Endianness is zeroed.");
                //             panic!("none.")
                //         }
                //         Endianness::LSB => self.0.to_le_bytes(),
                //         Endianness::MSB => self.0.to_be_bytes(),
                //         Endianness::Number => {
                //             $crate::info!("Endianness is invalid number.");
                //             panic!("none.")
                //         }
                //         Endianness::Undefined => {
                //             $crate::info!("Endianness is undefined.");
                //             panic!("none.")
                //         }
                //     }
                // }

                // fn from_bytes(
                //     bytes: [u8; Self::SIZE_BYTES],
                //     endianness: $crate::dtype::Endianness,
                // ) -> Self {
                //     match endianness {
                //         Endianness::TODO => {
                //             $crate::info!("Endianness is TODO");
                //             panic!("TODO");
                //         }
                //         Endianness::None => {
                //             $crate::info!("Endianness is zeroed.");
                //             panic!("none.")
                //         }
                //         Endianness::LSB => Self(<$inner>::from_le_bytes(bytes)),
                //         Endianness::MSB => Self(<$inner>::from_be_bytes(bytes)),
                //         Endianness::Number => {
                //             $crate::info!("Endianness is invalid number.");
                //             panic!("none.")
                //         }
                //         Endianness::Undefined => {
                //             $crate::info!("Endianness is undefined.");
                //             panic!("none.")
                //         }
                //     }
                // }
        )*
    };
}
pub use file_format_elf_dtype_impl;

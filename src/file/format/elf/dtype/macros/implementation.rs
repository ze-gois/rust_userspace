#[macro_export]
macro_rules! file_format_elf_dtype_impl {
    ($($dtype:ident),*) => {
        $crate::trait_implement_primitive!(false, $($dtype),*);
        $crate::trait_implement_bytes!($($dtype),*);
        $(
            impl $dtype {
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

                // pub fn read(fd: isize, endianness: $crate::dtype::Endianness) -> $crate::Result {
                //     let mut bytes = [0u8; <$dtype>::SIZE_BYTES];
                //     const INNER_SIZE: usize = <$dtype>::SIZE_BYTES;
                //     match syscall::read(fd, bytes.as_mut_ptr(), <$dtype>::SIZE_BYTES) {
                //         Ok((INNER_SIZE, _)) => {
                //             let value = match endianness {
                //                 Endianness::TODO => {
                //                     $crate::info!("Endianness is TODO");
                //                     panic!("TODO");
                //                 }
                //                 Endianness::None => {
                //                     $crate::info!("Endianness is zeroed.");
                //                     panic!("none.")
                //                 }
                //                 Endianness::LSB => <$inner>::from_le_bytes(bytes),
                //                 Endianness::MSB => <$inner>::from_be_bytes(bytes),
                //                 Endianness::Number => {
                //                     $crate::info!("Endianness is invalid number.");
                //                     panic!("none.")
                //                 }
                //                 Endianness::Undefined => {
                //                     $crate::info!("Endianness is undefined.");
                //                     panic!("none.")
                //                 }
                //             };
                //             Ok($crate::Ok::dtype::$dtype(value))
                //         }
                //         Ok(_) => Err($crate::dtype::Error::$dtype(0).to_k()), // usize::MAX - payload required.
                //         Err(_) => Err($crate::dtype::Error::$dtype(0).to_k()),
                //     }
                // }
            }
        )*
    };
}
pub use file_format_elf_dtype_impl;

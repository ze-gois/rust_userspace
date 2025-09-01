#[macro_export]
macro_rules! file_format_elf_dtype_impl {
    ($class:ident, $class_ident:ident, $($dtype:ident),*) => {
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
        // )*
    };
}
pub use file_format_elf_dtype_impl;

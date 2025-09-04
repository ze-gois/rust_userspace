#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

ample::r#struct!(
    pub Origin {}
);

#[macro_use]
pub mod target;
pub mod examples;
pub mod file;
pub mod image;
pub mod license;
pub mod memory;
pub mod panic;
pub mod result;
pub use result::{Error, Ok, Result};
pub mod traits;

ample::trait_implement_primitives!();
// impl<F: traits::Bytes<crate::target::Origin, crate::target::Origin>>
//     traits::Bytes<crate::Origin, crate::Origin> for F
// {
//     const BYTES_SIZE: usize = F::BYTES_SIZE;
//     fn from_bytes(
//         bytes: [u8; <Self as traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE],
//         endianness: bool,
//     ) -> Self
//     where
//         [u8; <Self as traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE]:,
//         [(); Self::BYTES_SIZE]:,
//     {
//         let target_bytes =
//             [0u8; <F as traits::Bytes<crate::target::Origin, crate::target::Origin>>::BYTES_SIZE];
//         F::from_bytes(target_bytes, endianness)
//     }
//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
//         // F::to_bytes(&self, endianness)
//         [0u8; Self::BYTES_SIZE]
//     }
// }

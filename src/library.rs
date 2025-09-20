#![no_std]
#![allow(incomplete_features)]
#![allow(unused_assignments)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

ample::r#struct!(
    #[derive(Debug)]
    pub struct Origin {}
);

#[macro_use]
pub mod target;
pub mod file;
pub mod license;
pub mod memory;
pub mod panic;
pub mod result;
pub use result::{Error, Ok, Result};
pub mod traits;

ample::trait_implement_primitives!();

// impl ample::traits::Bytes<crate::Origin, crate::Origin> for u8 {
//     const BYTES_SIZE: usize = 1;
//     const BYTES_ALIGN: usize = 1;

//     fn from_bytes(
//         bytes: [u8; <Self as crate::traits::Bytes<Origin, Origin>>::BYTES_SIZE],
//         _endianness: bool,
//     ) -> Self {
//         bytes[0]
//     }

//     fn to_bytes(
//         &self,
//         _endianness: bool,
//     ) -> [u8; <Self as crate::traits::Bytes<Origin, Origin>>::BYTES_SIZE] {
//         [*self]
//     }
// }

// ample::trait_bytes_naturalize!(ample, crate);

// impl<A: ample::traits::Bytes<ample::Origin>> crate::traits::Bytes<Origin, Destination> for A {
//     const BYTES_SIZE: usize = A::BYTES_SIZE;
//     const BYTES_ALIGN: usize = A::BYTES_ALIGN;

//     fn from_bytes(
//         bytes: [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE],
//         endianness: bool,
//     ) -> Self
//     where
//         [u8; <A as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE]:,
//     {
//         let mut ample_bytes = [0u8; <A as ample::traits::Bytes<ample::Origin>>::BYTES_SIZE];
//         ample_bytes.copy_from_slice(&bytes);
//         <A as ample::traits::Bytes<ample::Origin>>::from_bytes(ample_bytes, endianness)
//     }

//     fn to_bytes(
//         &self,
//         endianness: bool,
//     ) -> [u8; <Self as crate::traits::Bytes<Origin, Destination>>::BYTES_SIZE] {
//         <A as ample::traits::Bytes<ample::Origin>>::to_bytes(self, endianness)
//     }
// }

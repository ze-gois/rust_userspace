#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
// #![feature(never_type)]

ample::r#struct!(
    #[derive(Debug)]
    pub struct Origin {}
);

pub mod target;
pub mod file;
pub mod memory;
pub mod panic;
pub mod result;
pub mod traits;

pub use result::{Error, Ok, Result};

ample::trait_implement_primitives!();

// impl ample::traits::Bytes<crate::Origin, crate::Origin> for u8 {
//     const REPRESENTATION_SIZE: usize = 1;

//     fn from_bytes(
//         bytes: [u8; <Self as crate::traits::Bytes<Origin, Origin>>::REPRESENTATION_SIZE],
//         _endianness: bool,
//     ) -> Self {
//         bytes[0]
//     }

//     fn to_bytes(
//         &self,
//         _endianness: bool,
//     ) -> [u8; <Self as crate::traits::Bytes<Origin, Origin>>::REPRESENTATION_SIZE] {
//         [*self]
//     }
// }

// ample::trait_bytes_naturalize!(ample, crate);

// impl<A: ample::traits::Bytes<ample::Origin>> crate::traits::Bytes<Origin, Destination> for A {
//     const REPRESENTATION_SIZE: usize = A::REPRESENTATION_SIZE;

//     fn from_bytes(
//         bytes: [u8; <Self as crate::traits::Bytes<Origin, Destination>>::REPRESENTATION_SIZE],
//         endianness: bool,
//     ) -> Self
//     where
//         [u8; <A as ample::traits::Bytes<ample::Origin>>::REPRESENTATION_SIZE]:,
//     {
//         let mut ample_bytes = [0u8; <A as ample::traits::Bytes<ample::Origin>>::REPRESENTATION_SIZE];
//         ample_bytes.copy_from_slice(&bytes);
//         <A as ample::traits::Bytes<ample::Origin>>::from_bytes(ample_bytes, endianness)
//     }

//     fn to_bytes(
//         &self,
//         endianness: bool,
//     ) -> [u8; <Self as crate::traits::Bytes<Origin, Destination>>::REPRESENTATION_SIZE] {
//         <A as ample::traits::Bytes<ample::Origin>>::to_bytes(self, endianness)
//     }
// }

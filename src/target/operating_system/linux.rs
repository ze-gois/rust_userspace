#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
pub struct Origin;

pub mod syscall;

pub use crate::info;

pub mod result;
pub use result::{Error, Ok, Result};

// #[macro_use]
// pub mod macros;

// impl<F: traits::Primitive<crate::target::Origin, crate::target::Origin>>
//     traits::Primitive<crate::Origin, crate::Origin> for F
// {
//     const IS_PRIMITIVE: bool =
//         <Self as traits::Primitive<crate::target::Origin, crate::target::Origin>>::IS_PRIMITIVE;
// }

// impl<F: traits::Bytes<crate::target::Origin, crate::target::Origin>>
//     traits::Bytes<crate::Origin, crate::Origin> for F
// {
//     const BYTES_SIZE: usize =
//         <Self as traits::Bytes<crate::target::Origin, crate::target::Origin>>::BYTES_SIZE;

//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//     where
//         [(); Self::BYTES_SIZE]:,
//     {
//         <Self as traits::Bytes<crate::target::Origin, crate::target::Origin>>::from_bytes(bytes, endianness)
//     }

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE]
//     where
//         [(); Self::BYTES_SIZE]:,
//     {
//         <Self as traits::Bytes<crate::target::Origin, crate::target::Origin>>::to_bytes(self, endianness)
//     }
// }

// macro_rules! naturalize {
//     ($($trait:tt)::*, $($origin:ident),*) => {};
// }

// naturalize!(traits::Bytes, arch, human);
// pub use naturalize;

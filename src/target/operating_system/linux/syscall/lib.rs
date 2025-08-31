#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
pub struct Origin;

pub use human::info;

pub mod result;
pub use result::{Error, Ok, Result};

#[macro_use]
pub mod macros;

publishing!(
    [Syscall3; 0; READ; read ; "Read"],
    [Syscall3; 1; WRITE; write ; "Write"],
    [Syscall3; 2; OPEN; open ; "Open"],
    [Syscall1; 3; CLOSE; close ; "Close"],
    [Syscall2; 5; FSTAT; fstat ; "FStat"],
    [Syscall3; 8; LSEEK; lseek ; "LSeek"],
    [Syscall6; 9; MMAP; mmap ; "MMap"],
    [Syscall3; 10; MPROTECT; mprotect ; "MProtect"],
    [Syscall2; 11; MUNMAP; munmap ; "MUnMap"],
    [Syscall1; 60; EXIT; exit ; "Exit"],
    [Syscall3; 257; OPENAT; openat ; "OpenAt"],
    [Syscall4; 258; OPENAT4; openat4; "OpenAt4"]
);

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
pub use naturalize;

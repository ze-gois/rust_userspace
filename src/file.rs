pub mod load;
pub use load::load;

pub mod information;
pub use information::information;

pub mod print;
pub use print::print;

pub mod open;
pub use open::open;

pub mod seek;
pub use seek::seek;

pub mod format;

pub mod result;
pub use result::{Error, Ok, Result};

// impl traits::Bytes<Origin, Origin> for &str {
//     const BYTES_SIZE: usize = core::mem::size_of::<&str>();
//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
//         let bytes =
//         memory::alloc::<char>(self.len());
//     }
// }

ample::r#struct!(
    pub struct Information {
        size: usize,
    }
);

ample::r#struct!(
    pub struct File {
        pub descriptor: isize,
        pub information: Information, // pub memory : Option<*const u8>
    }
);

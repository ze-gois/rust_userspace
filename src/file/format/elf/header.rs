pub const EI_NIDENT: usize = 16;

pub mod identifier;
pub use identifier::Identifier;

pub mod etype;
pub mod flags;
pub mod machine;
pub mod version;

pub mod class_32;
pub use class_32::Header32;

pub mod class_64;
pub use class_64::Header64;

pub mod result;
pub use result::{Error, Ok, Result};

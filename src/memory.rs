pub mod alloc;
pub mod heap;
pub mod page;
pub mod result;
pub mod stack;

pub use result::{Error, Ok};
pub use stack::Stack;

pub struct Origin {}

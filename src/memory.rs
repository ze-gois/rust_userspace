pub mod alloc;
pub mod heap;
pub mod page;
pub mod result;
pub mod stack;
// pub use allocate::allocate;

pub use result::{Error, Ok};
pub use stack::Stack;
// pub mod macros;

pub struct Origin {}

// trait_implement_primitives!();

pub mod str;
pub use str::Str;

pub trait Ample {}
pub trait Userspace {}

impl<A: ample::traits::Bytes<ample::Origin, ample::Origin>> Ample for A {}
impl<U: ample::traits::Bytes<crate::Origin, crate::Origin>> Userspace for U {}

/// Linux mapping-sharing selector encoded in the low `mmap(2)` flag bits.
///
/// This is a selector field rather than an independent set of flags.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sharing(i32);

impl Sharing {
    pub const SHARED: Self = Self(MAP_SHARED);
    pub const PRIVATE: Self = Self(MAP_PRIVATE);
    pub const SHARED_VALIDATE: Self = Self(MAP_SHARED_VALIDATE);

    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

pub const MAP_SHARED: i32 = 0x01;
pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_SHARED_VALIDATE: i32 = 0x03;
pub const MAP_TYPE: i32 = 0x0f;

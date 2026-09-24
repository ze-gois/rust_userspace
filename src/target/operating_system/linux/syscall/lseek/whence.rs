/// Linux `lseek(2)` origin selector.
///
/// Kept as a transparent value rather than a closed Rust enum so unknown or
/// future kernel values remain representable.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Whence(i32);

impl Whence {
    pub const SET: Self = Self(SEEK_SET);
    pub const CURRENT: Self = Self(SEEK_CUR);
    pub const END: Self = Self(SEEK_END);
    pub const DATA: Self = Self(SEEK_DATA);
    pub const HOLE: Self = Self(SEEK_HOLE);

    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4;

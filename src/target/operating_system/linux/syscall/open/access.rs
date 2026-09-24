/// Linux file access mode encoded in the low bits of `open(2)` flags.
///
/// This is a selector field, not a bitset. The raw value is preserved so future
/// or unknown kernel values remain representable.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Access(i32);

impl Access {
    pub const READ_ONLY: Self = Self(O_RDONLY);
    pub const WRITE_ONLY: Self = Self(O_WRONLY);
    pub const READ_WRITE: Self = Self(O_RDWR);

    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

pub const O_RDONLY: i32 = 0o0;
pub const O_WRONLY: i32 = 0o1;
pub const O_RDWR: i32 = 0o2;
pub const O_ACCMODE: i32 = 0o3;

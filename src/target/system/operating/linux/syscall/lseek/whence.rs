#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Whence {
    Set = 0,
    Current = 1,
    End = 2,
}

impl Whence {
    pub const fn to(self) -> i32 {
        self as i32
    }
}

pub const SEEK_SET: i32 = Whence::Set as i32;
pub const SEEK_CUR: i32 = Whence::Current as i32;
pub const SEEK_END: i32 = Whence::End as i32;

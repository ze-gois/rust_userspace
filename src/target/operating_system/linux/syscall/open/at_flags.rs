#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AtFlag(isize);

impl AtFlag {
    pub const CURRENT_WORKING_DIRECTORY: Self = Self(AT_FDCWD);
    pub const REMOVE_DIRECTORY: Self = Self(AT_REMOVEDIR);
    pub const SYMBOLIC_LINK_FOLLOW: Self = Self(AT_SYMLINK_FOLLOW);
    pub const SYMBOLIC_LINK_NO_FOLLOW: Self = Self(AT_SYMLINK_NOFOLLOW);

    pub const fn from_raw(raw: isize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> isize {
        self.0
    }
}

pub const AT_FDCWD: isize = -100;
pub const AT_REMOVEDIR: isize = 0x200;
pub const AT_SYMLINK_FOLLOW: isize = 0x400;
pub const AT_SYMLINK_NOFOLLOW: isize = 0x100;

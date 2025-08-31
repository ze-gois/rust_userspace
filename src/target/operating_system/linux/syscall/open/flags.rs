#[repr(isize)]
#[derive(Clone, Copy)]
pub enum Flag {
    // Access modes
    RDONLY = 0o0,
    WRONLY = 0o1,
    RDWR = 0o2,

    // Creation and file status flags
    CREAT = 0o100,
    EXCL = 0o200,
    NOCTTY = 0o400,
    TRUNC = 0o1000,
    APPEND = 0o2000,
    NONBLOCK = 0o4000,
    DSYNC = 0o10000,
    SYNC = 0o4010000,
    DIRECTORY = 0o200000,
    NOFOLLOW = 0o400000,
    CLOEXEC = 0o2000000,
}

impl Into<usize> for Flag {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<i32> for Flag {
    fn into(self) -> i32 {
        self as i32
    }
}

#[repr(isize)]
#[derive(Clone, Copy)]
pub enum AtFlag {
    // Special fd values for openat
    FDCWD = -100,            // Use current working directory
    REMOVEDIR = 0x200,       // Remove directory instead of file
    SymlinkFollow = 0x400,   // Follow symbolic links
    SymlinkNoFollow = 0x100, // Don't follow symbolic links
}

impl Into<usize> for AtFlag {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<isize> for AtFlag {
    fn into(self) -> isize {
        self as isize
    }
}

// Flag | AtFlag
impl core::ops::BitOr<AtFlag> for Flag {
    type Output = usize;
    fn bitor(self, rhs: AtFlag) -> usize {
        (self as usize) | (rhs as usize)
    }
}

// AtFlag | Flag
impl core::ops::BitOr<Flag> for AtFlag {
    type Output = usize;
    fn bitor(self, rhs: Flag) -> usize {
        (self as usize) | (rhs as usize)
    }
}

// Flag | Flag
impl core::ops::BitOr for Flag {
    type Output = usize;
    fn bitor(self, rhs: Self) -> usize {
        (self as usize) | (rhs as usize)
    }
}

// AtFlag | AtFlag
impl core::ops::BitOr for AtFlag {
    type Output = usize;
    fn bitor(self, rhs: Self) -> usize {
        (self as usize) | (rhs as usize)
    }
}

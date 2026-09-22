#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,        // dev_t
    pub st_ino: u64,        // ino_t
    pub st_nlink: u64,      // nlink_t
    pub st_mode: u32,       // mode_t
    pub st_uid: u32,        // uid_t
    pub st_gid: u32,        // gid_t
    pub __pad0: i32,        // padding
    pub st_rdev: u64,       // dev_t (special files)
    pub st_size: i64,       // off_t
    pub st_blksize: i64,    // blksize_t
    pub st_blocks: i64,     // blkcnt_t
    pub st_atime: i64,      // time_t (seconds)
    pub st_atime_nsec: i64, // nanoseconds
    pub st_mtime: i64,      // time_t
    pub st_mtime_nsec: i64, // nanoseconds
    pub st_ctime: i64,      // time_t
    pub st_ctime_nsec: i64, // nanoseconds
    pub __unused: [i64; 3], // reserved/padding
}

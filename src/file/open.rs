use crate::target::os::syscall;

pub fn open(filepath: &str) -> isize {
    match filepath.chars().nth(0) {
        _ => {
            let dfd = syscall::open::AtFlag::FDCWD as isize;
            let flag = syscall::open::Flag::RDONLY as i32;
            let fd = match syscall::openat(dfd, filepath.as_ptr(), flag) {
                core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                    crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                        crate::target::os::syscall::open::Ok::Default(m),
                    )),
                ))) => m as isize,
                _ => -1,
            };
            fd
        }
    }
}

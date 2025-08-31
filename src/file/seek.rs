use crate::target::os::syscall;

pub fn seek(file_descriptor: isize, offset: i64) -> usize {
    match syscall::lseek(
        file_descriptor as i32,
        offset,
        syscall::lseek::Flag::SET as i32,
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::LSeek(
                crate::target::os::syscall::lseek::Ok::Default(m),
            )),
        ))) => m,
        _ => 0,
    }
}

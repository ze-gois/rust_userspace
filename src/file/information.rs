use crate::target::os::syscall;

pub fn from_fd(fd: isize) -> syscall::fstat::Stat {
    let p = match syscall::mmap(
        core::ptr::null_mut(),
        core::mem::size_of::<syscall::fstat::Stat>(),
        (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
        (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Shared) as i32,
        -1,
        0,
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                crate::target::os::syscall::mmap::Ok::Default(m),
            )),
        ))) => m as *const u8,
        _ => {
            crate::info!("Failed to mmap file");
            panic!("ohones")
        }
    };

    let p = p as *const syscall::fstat::Stat;
    let Ok(_) = syscall::fstat(fd as isize, p).map_err(|_e| panic!("natv"));

    unsafe { *p }
}

pub fn from_path(filepath: &str) -> syscall::fstat::Stat {
    let file_descriptor = crate::file::open(filepath);
    from_fd(file_descriptor)
}

pub fn information(_file: &mut crate::file::File) -> bool {
    false
}

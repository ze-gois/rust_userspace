pub fn open(file_path: &str) -> isize {
    let file_path =
        ample::string::terminate::<crate::memory::heap::Allocator>(file_path);

    if file_path.is_null() {
        return -1;
    }

    match crate::target::os::syscall::openat(
        crate::target::os::syscall::open::AtFlag::FDCWD.to(),
        file_path,
        crate::target::os::syscall::open::Flag::RDONLY.to(),
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::OPENAT(fd),
            )),
        ))) => fd as isize,
        _ => -1,
    }
}

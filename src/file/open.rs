pub fn open(file_path: &str) -> isize {
    use ample::traits::AllocatableResult;
    let file_path =
        ample::string::terminate::<crate::Origin, crate::Origin, crate::memory::heap::Allocator>(
            file_path,
        );

    let file_path = match file_path {
        core::result::Result::Ok(a) => a.as_ptr(),
        _ => return -1,
    };

    match crate::target::os::syscall::openat(
        crate::target::os::syscall::open::AtFlag::FDCWD.to(),
        file_path,
        crate::target::os::syscall::open::Flag::RDONLY.to(),
    ) {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::OPENAT(fd),
            )),
        ))) => fd as isize,
        _ => -1,
    }
}

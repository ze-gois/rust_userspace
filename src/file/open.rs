pub fn open(file_path: &str) -> isize {
    let file_path = ample::string::terminate::<crate::memory::heap::Allocator>(file_path);

    if file_path.is_null() {
        return -1;
    }

    match crate::target::os::syscall::openat(
        crate::target::os::syscall::openat::CURRENT_WORKING_DIRECTORY,
        file_path,
        crate::target::os::syscall::open::Flag::RDONLY.to(),
        0,
    ) {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::OpenAt(
                    crate::target::os::syscall::openat::Ok::Default(file_descriptor),
                ),
            )),
        )) => file_descriptor as isize,
        _ => -1,
    }
}

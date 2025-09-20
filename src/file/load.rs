use crate::target::os::syscall;
pub fn load(filepath: &str) -> Option<(isize, syscall::fstat::Stat, *const u8)> {
    use ample::traits::AllocatableResult;
    let filepath =
        ample::string::terminate::<crate::Origin, crate::Origin, crate::memory::heap::Allocator>(
            filepath,
        );

    let filepath = match filepath {
        core::result::Result::Ok(a) => a.as_ptr(),
        _ => return None,
    };

    let license_mapping;
    'opening: loop {
        #[allow(unused_assignments)]
        let mut fd: isize = isize::MIN;
        let stat;
        'closing: loop {
            fd = match syscall::openat(
                syscall::open::AtFlag::FDCWD.to(),
                filepath,
                syscall::open::Flag::RDONLY.to(),
            ) {
                core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                    crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                        crate::target::os::syscall::open::Ok::OPENAT(fd),
                    )),
                ))) => fd as isize,
                _ => break 'opening None,
            };

            stat = crate::file::information::from_fd(fd);

            license_mapping = match syscall::mmap(
                core::ptr::null_mut(),
                stat.st_size as usize,
                (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
                (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
                -1,
                0,
            ) {
                core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                    crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                        crate::target::os::syscall::mmap::Ok::Default(fd),
                    )),
                ))) => fd as *const u8,
                _ => {
                    crate::info!("Failed to mmap file");
                    panic!("k")
                }
            };

            let _ = syscall::read(fd, license_mapping, stat.st_size as usize);
            break 'closing;
        }
        if fd >= 0 {
            // let _ = syscall::close(fd as i32);
            break 'opening Some((fd, stat, license_mapping));
        }
        break 'opening None;
    }
}

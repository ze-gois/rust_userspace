use crate::memory::heap::Allocator;
use crate::target::os::syscall;

pub fn load(filepath: &str) -> Option<(isize, syscall::fstat::Stat, *const u8)> {
    let filepath = ample::string::terminate::<Allocator>(filepath);
    if filepath.is_null() {
        return None;
    }

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

            license_mapping = Allocator::allocate::<u8>(stat.st_size as usize);

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

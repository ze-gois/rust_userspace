pub use ample::traits::Bytes;
impl ample::traits::Allocatable<ample::Origin, ample::Origin> for u8 {
    unsafe fn allocate(layout: ample::traits::allocatable::Layout) -> *mut u8 {
        let n =
            layout.size * <u8 as ample::traits::Bytes<ample::Origin, ample::Origin>>::BYTES_SIZE;
        let aligned_size = n + layout.align; //* t_size; //(t_size + ::memory::page::SIZE - 1) & !(::memory::page::SIZE - 1);

        match crate::target::os::syscall::mmap(
            core::ptr::null_mut(),
            aligned_size,
            (crate::target::os::syscall::mmap::Prot::Read
                | crate::target::os::syscall::mmap::Prot::Write) as i32,
            (crate::target::os::syscall::mmap::Flag::Anonymous
                | crate::target::os::syscall::mmap::Flag::Private) as i32,
            -1,
            0,
        ) {
            core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                    crate::target::os::syscall::mmap::Ok::Default(m),
                )),
            ))) => m as *mut u8,
            _ => panic!("Failed to allocate memory"),
        }
    }

    unsafe fn deallocate(ptr: *mut u8, layout: ample::traits::allocatable::Layout) {
        let total_size =
            layout.size * <u8 as ample::traits::Bytes<ample::Origin, ample::Origin>>::BYTES_SIZE;

        let aligned_size =
            (total_size + crate::memory::page::SIZE - 1) & !(crate::memory::page::SIZE - 1);

        let _ = crate::target::os::syscall::munmap(ptr, aligned_size);
    }
}

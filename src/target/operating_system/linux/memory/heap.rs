use core::alloc::Layout;

unsafe impl ample::traits::Allocating for crate::memory::heap::Allocator {
    fn allocate(layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        if layout.align() > crate::memory::page::BASE_SIZE {
            return core::ptr::null_mut();
        }

        const MAP_ANONYMOUS: i32 = 0x20;

        match crate::target::os::syscall::mmap(
            core::ptr::null_mut(),
            layout.size(),
            crate::target::os::syscall::mmap::protection::PROT_READ
                | crate::target::os::syscall::mmap::protection::PROT_WRITE,
            crate::target::os::syscall::mmap::sharing::MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        ) {
            core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
                crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                    crate::target::os::syscall::mmap::Ok::Default(address),
                )),
            ))) => address as *mut u8,
            _ => core::ptr::null_mut(),
        }
    }

    unsafe fn deallocate(pointer: *mut u8, layout: Layout) -> bool {
        if layout.size() == 0 {
            return true;
        }

        if pointer.is_null() {
            return false;
        }

        crate::target::os::syscall::munmap(pointer, layout.size()).is_ok()
    }
}

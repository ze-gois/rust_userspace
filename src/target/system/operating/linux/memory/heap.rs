use core::alloc::Layout;

unsafe impl ample::traits::Allocating for crate::memory::heap::Allocator {
    fn allocate(layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        if layout.align() > crate::memory::page::SIZE {
            return core::ptr::null_mut();
        }

        match crate::target::system::operating::linux::syscall::mmap(
            core::ptr::null_mut(),
            layout.size(),
            (crate::target::system::operating::linux::syscall::mmap::Protection::Read
                | crate::target::system::operating::linux::syscall::mmap::Protection::Write)
                as i32,
            (crate::target::system::operating::linux::syscall::mmap::Flag::Anonymous
                | crate::target::system::operating::linux::syscall::mmap::Flag::Private)
                as i32,
            -1,
            0,
        ) {
            core::result::Result::Ok(crate::Ok::Target(
                crate::target::Ok::OperatingSystem(
                    crate::target::system::operating::linux::Ok::Syscall(
                        crate::target::system::operating::linux::syscall::Ok::Mmap(
                            crate::target::system::operating::linux::syscall::mmap::Ok::Default(
                                pointer,
                            ),
                        ),
                    ),
                ),
            )) => pointer as *mut u8,
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

        crate::target::system::operating::linux::syscall::munmap(pointer, layout.size()).is_ok()
    }
}

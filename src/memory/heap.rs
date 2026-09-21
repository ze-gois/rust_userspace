use core::alloc::Layout;

#[derive(Debug, Clone, Copy, Default)]
pub struct Allocator;

unsafe impl ample::traits::Allocating for Allocator {
    fn allocate(layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        if layout.align() > crate::memory::page::SIZE {
            return core::ptr::null_mut();
        }

        match crate::target::os::syscall::mmap(
            core::ptr::null_mut(),
            layout.size(),
            (crate::target::os::syscall::mmap::Prot::Read
                | crate::target::os::syscall::mmap::Prot::Write) as i32,
            (crate::target::os::syscall::mmap::Flag::Anonymous
                | crate::target::os::syscall::mmap::Flag::Private) as i32,
            -1,
            0,
        ) {
            core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MMap(
                    crate::target::os::syscall::mmap::Ok::Default(pointer),
                )),
            ))) => pointer as *mut u8,
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

        match crate::target::os::syscall::munmap(pointer, layout.size()) {
            core::result::Result::Ok(_) => true,
            core::result::Result::Err(_) => false,
        }
    }
}

impl Allocator {
    pub fn allocate<T>(count: usize) -> *mut T {
        let Ok(layout) = Layout::array::<T>(count) else {
            return core::ptr::null_mut();
        };

        <Self as ample::traits::Allocating>::allocate(layout) as *mut T
    }

    /// # Safety
    ///
    /// `pointer` must have been returned by `Allocator::allocate::<T>` for
    /// the same `count`, and all initialized values must have been dropped.
    pub unsafe fn deallocate<T>(pointer: *mut T, count: usize) -> bool {
        let Ok(layout) = Layout::array::<T>(count) else {
            return false;
        };

        unsafe {
            <Self as ample::traits::Allocating>::deallocate(pointer as *mut u8, layout)
        }
    }
}

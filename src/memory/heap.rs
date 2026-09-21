use ample::traits::AllocatableResult;

use crate::Origin;
use core::alloc::Layout;

ample::r#struct!(
    #[derive(Debug)]
    pub struct Allocator {}
);

pub type AllocatorPointer = *mut Allocator;

impl ample::traits::Allocatable<Origin> for Allocator {
    type Ok = crate::Ok;
    type Error = crate::Error;
    fn allocate(numerosity_of_bytes: usize) -> crate::Result {
        match crate::target::os::syscall::mmap(
            core::ptr::null_mut(),
            numerosity_of_bytes,
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
            ))) => core::result::Result::Ok(crate::Ok::Memory(crate::memory::Ok::HeapAllocate(
                m as *mut Self,
            ))),
            _ => panic!("Failed to allocate memory"),
        }
    }

    fn deallocate(ptr: *mut Self, numerosity_of_bytes: usize) -> crate::Result {
        match crate::target::os::syscall::munmap(ptr as *mut u8, numerosity_of_bytes) {
            _ => true,
        };
        core::result::Result::Ok(crate::Ok::Memory(crate::memory::Ok::HeapAllocate(
            ptr as *mut Self,
        )))
    }
}

unsafe impl ample::traits::Allocating for Allocator {
    fn allocate(layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        if layout.align() > crate::memory::page::SIZE {
            return core::ptr::null_mut();
        }

        match <Allocator as ample::traits::Allocatable<Origin>>::allocate(layout.size()) {
            Ok(result) => result.as_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn deallocate(pointer: *mut u8, layout: Layout) -> bool {
        if layout.size() == 0 {
            return true;
        }

        if pointer.is_null() {
            return false;
        }

        match <Allocator as ample::traits::Allocatable<Origin>>::deallocate(
            pointer as *mut Allocator,
            layout.size(),
        ) {
            Ok(_) => true,
            Err(_) => false,
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

// pub type StringAllocator = *const u8;

// ample::result!(
//     Ok;
//     "Allocate Ok";
//     usize;
//     [
//         [1; USERSPACE_MEMORY_ALLOCATION_HEAP_DEFAULT_OK; Default; AllocatorPointer; "ZE"; "Entry to ze"],
//         [2; USERSPACE_MEMORY_ALLOCATION_HEAP_ALLOCATOR_DEFAULT_OK; Allocator; AllocatorPointer; "ZE"; "Entry to ze"],
//         [3; USERSPACE_MEMORY_ALLOCATION_HEAP_DEALLOCATOR_DEFAULT_OK; Deallocator; AllocatorPointer; "ZE"; "Entry to ze"],
//         [4; USERSPACE_MEMORY_ALLOCATION_HEAP_STRING_DEFAULT_OK; String; StringAllocator; "ZE"; "Entry to ze"],
//         // [2; USERSPACE_MEMORY_ALLOCATION_HEAP_ALLOCATING_OK; Allocator; crate::memory::Ok; "ZE"; "Entry to ze"],
//     ];
//     Error;
//     "Allocator Ok";
//     usize;
//     [
//         [1; USERSPACE_MEMORY_ALLOCATION_HEAP_DEFAULT_ERROR; Default; AllocatorPointer; "ZE"; "Entry to ze"],
//         [2; USERSPACE_MEMORY_ALLOCATION_HEAP_ALLOCATOR_DEFAULT_ERROR; Allocator; AllocatorPointer; "ZE"; "Entry to ze"],
//         [3; USERSPACE_MEMORY_ALLOCATION_HEAP_DEALLOCATOR_DEFAULT_ERROR; Deallocator; AllocatorPointer; "ZE"; "Entry to ze"],
//     ]
// );

impl ample::traits::AllocatableResult for crate::Ok {
    fn as_ptr(&self) -> *mut u8 {
        match self {
            crate::Ok::Memory(crate::memory::Ok::HeapAllocate(m)) => *m as *mut u8,
            _ => core::ptr::null_mut(),
        }
    }

    fn from_raw(raw: *mut u8) -> Self {
        crate::Ok::Memory(crate::memory::Ok::HeapAllocate(raw as *mut Allocator))
    }
}

impl ample::traits::AllocatableResult for crate::Error {
    fn as_ptr(&self) -> *mut u8 {
        match self {
            _ => core::ptr::null_mut(),
            // core::result::Result::Err(crate::Error::Memory(crate::memory::Error::Allocate(
            //     crate::memory::heap::Error::Allocate(
            //         crate::memory::Error::Default(m),
            //     ),
            // ))) => *m as *mut u8,
        }
    }

    fn from_raw(_raw: *mut u8) -> Self {
        crate::Error::Error(3)
        // crate::Error::Memory(crate::memory::Error::Allocate(
        //     crate::memory::heap::Error::Allocate(
        //         crate::memory::Error::Default(core::ptr::null_mut()),
        //     ),
        // ))
    }
}

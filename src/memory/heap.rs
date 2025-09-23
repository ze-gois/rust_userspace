use ample::traits::AllocatableResult;

use crate::Origin;

ample::r#struct!(
    #[derive(Debug)]
    pub struct Allocator {}
);

// ample::traits_impl_blanket_bytes!(Allocator);

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

pub trait Allocating<T> {
    fn allocate(count: usize) -> *mut T;
    fn allocate_slice(count: usize) -> &'static mut [T];
    fn deallocate(ptr: *mut T, count: usize) -> bool;
    fn deallocate_slice(slice: &mut [T]) -> bool;
}

impl<T> Allocating<T> for T
where
    T: ample::traits::Bytes<Origin, Origin>,
    // T: Default,
{
    fn allocate(numerosity: usize) -> *mut T {
        let numerosity_of_bytes = numerosity * T::BYTES_SIZE + T::BYTES_ALIGN;
        match <Allocator as ample::traits::Allocatable<Origin>>::allocate(numerosity_of_bytes) {
            Ok(m) => m.as_ptr() as *mut T,
            _ => core::ptr::null_mut(),
        }
    }

    /// Deallocate an array previously allocated with allocate_array
    fn deallocate(ptr: *mut T, numerosity: usize) -> bool {
        let total_size = numerosity * T::BYTES_SIZE + T::BYTES_ALIGN;

        let aligned_size =
            (total_size + crate::memory::page::SIZE - 1) & !(crate::memory::page::SIZE - 1);

        // let _ = crate::target::os::syscall::munmap(ptr as *mut u8, aligned_size);
        // true
        match <Allocator as ample::traits::Allocatable<Origin>>::deallocate(
            ptr as *mut Allocator,
            aligned_size,
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Allocate and initialize a slice
    fn allocate_slice(numerosity: usize) -> &'static mut [T] {
        let ptr = Self::allocate(numerosity) as *mut T;

        // for i in 0..numerosity {
        //     unsafe {
        //         *ptr.add(i) = T::default();
        //     }
        // }

        unsafe { core::slice::from_raw_parts_mut(ptr, numerosity) }
    }

    fn deallocate_slice(slice: &mut [T]) -> bool {
        Self::deallocate(slice.as_mut_ptr() as *mut T, slice.len())
    }
}

impl<T> Allocating<T> for &[T]
where
    T: ample::traits::Bytes<Origin, Origin>,
    // T: Default,
{
    fn allocate(numerosity: usize) -> *mut T {
        match <Allocator as ample::traits::Allocatable<Origin>>::allocate(
            numerosity * T::BYTES_SIZE,
        ) {
            Ok(ptr) => ptr.as_ptr() as *mut T,
            Err(_) => core::ptr::null_mut(),
        }
    }

    /// Deallocate an array previously allocated with allocate_array
    fn deallocate(ptr: *mut T, numerosity: usize) -> bool {
        match <Allocator as ample::traits::Allocatable<Origin>>::deallocate(
            ptr as *mut Allocator,
            numerosity,
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Allocate and initialize a slice
    fn allocate_slice(numerosity: usize) -> &'static mut [T] {
        let ptr = match <Allocator as ample::traits::Allocatable<Origin>>::allocate(numerosity) {
            Ok(ptr) => ptr.as_ptr() as *mut T,
            Err(_) => core::ptr::null_mut(),
        };

        // for i in 0..numerosity {
        //     unsafe {
        //         *ptr.add(i) = T::default();
        //     }
        // }

        unsafe { core::slice::from_raw_parts_mut(ptr, numerosity) }
    }

    fn deallocate_slice(slice: &mut [T]) -> bool {
        match <Allocator as ample::traits::Allocatable<Origin>>::deallocate(
            slice.as_mut_ptr() as *mut Allocator,
            slice.len(),
        ) {
            Ok(_) => true,
            Err(_) => false,
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

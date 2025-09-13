use crate::Origin;
use ample::traits::Allocatable;
use ample::traits::Bytes;

/// A concrete allocator type that uses Linux system calls
#[derive(Debug, Clone, Copy)]
pub struct Allocator;

impl Allocatable<Origin> for Allocator {
    fn allocate(numerosity_of_bytes: usize) -> *mut Self {
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
            ))) => m as *mut Self,
            _ => panic!("Failed to allocate memory"),
        }
    }

    fn deallocate(ptr: *mut Self, numerosity_of_bytes: usize) -> bool {
        match crate::target::os::syscall::munmap(ptr as *mut u8, numerosity_of_bytes) {
            _ => true,
        }
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
    T: Bytes<Origin>,
    T: Default,
{
    fn allocate(numerosity: usize) -> *mut T {
        let numerosity_of_bytes = numerosity * T::BYTES_SIZE + T::BYTES_ALIGN;
        <Allocator as Allocatable<Origin>>::allocate(numerosity_of_bytes) as *mut T
    }

    /// Deallocate an array previously allocated with allocate_array
    fn deallocate(ptr: *mut T, numerosity: usize) -> bool {
        let total_size = numerosity * T::BYTES_SIZE + T::BYTES_ALIGN;

        let aligned_size =
            (total_size + crate::memory::page::SIZE - 1) & !(crate::memory::page::SIZE - 1);

        // let _ = crate::target::os::syscall::munmap(ptr as *mut u8, aligned_size);
        // true
        <Allocator as Allocatable<Origin>>::deallocate(ptr as *mut Allocator, aligned_size)
    }

    /// Allocate and initialize a slice
    fn allocate_slice(numerosity: usize) -> &'static mut [T] {
        let ptr = Self::allocate(numerosity) as *mut T;

        for i in 0..numerosity {
            unsafe {
                *ptr.add(i) = T::default();
            }
        }

        unsafe { core::slice::from_raw_parts_mut(ptr, numerosity) }
    }

    fn deallocate_slice(slice: &mut [T]) -> bool {
        Self::deallocate(slice.as_mut_ptr() as *mut T, slice.len())
    }
}

impl<T> Allocating<T> for &[T]
where
    T: Bytes<Origin>,
    T: Default,
{
    fn allocate(numerosity: usize) -> *mut T {
        <Allocator as Allocatable<Origin>>::allocate(numerosity * T::BYTES_SIZE) as *mut T
    }

    /// Deallocate an array previously allocated with allocate_array
    fn deallocate(ptr: *mut T, numerosity: usize) -> bool {
        <Allocator as Allocatable<Origin>>::deallocate(ptr as *mut Allocator, numerosity)
    }

    /// Allocate and initialize a slice
    fn allocate_slice(numerosity: usize) -> &'static mut [T] {
        let ptr = <Allocator as Allocatable<Origin>>::allocate(numerosity) as *mut T;

        for i in 0..numerosity {
            unsafe {
                *ptr.add(i) = T::default();
            }
        }

        unsafe { core::slice::from_raw_parts_mut(ptr, numerosity) }
    }

    fn deallocate_slice(slice: &mut [T]) -> bool {
        <Allocator as Allocatable<Origin>>::deallocate(
            slice.as_mut_ptr() as *mut Allocator,
            slice.len(),
        )
    }
}

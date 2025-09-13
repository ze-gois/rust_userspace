use crate::Origin;
use ample::traits::Allocatable;
use ample::traits::Bytes;

/// A concrete allocator type that uses Linux system calls
#[derive(Debug, Clone, Copy)]
pub struct Allocator;

impl Allocatable<Origin> for Allocator {
    fn allocate(numerosity_of_bytes: usize) -> *mut Self {
        let ptr =
            crate::os::syscall::mmap(core::ptr::null_mut(), numerosity_of_bytes, 0x3, 0x22, -1, 0);
        ptr as *mut Self
    }

    fn deallocate(ptr: *mut Self, numerosity_of_bytes: usize) -> bool {
        let result = crate::os::syscall::munmap(ptr as *mut u8, numerosity_of_bytes);
        result == 0
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

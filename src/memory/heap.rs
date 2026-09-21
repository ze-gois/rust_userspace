use core::alloc::Layout;

#[derive(Debug, Clone, Copy, Default)]
pub struct Allocator;

impl Allocator {
    pub fn allocate<T>(count: usize) -> *mut T
    where
        Self: ample::traits::Allocating,
    {
        let Ok(layout) = Layout::array::<T>(count) else {
            return core::ptr::null_mut();
        };

        <Self as ample::traits::Allocating>::allocate(layout) as *mut T
    }

    /// # Safety
    ///
    /// `pointer` must have been returned by `Allocator::allocate::<T>` for
    /// the same `count`, and all initialized values must have been dropped.
    pub unsafe fn deallocate<T>(pointer: *mut T, count: usize) -> bool
    where
        Self: ample::traits::Allocating,
    {
        let Ok(layout) = Layout::array::<T>(count) else {
            return false;
        };

        unsafe {
            <Self as ample::traits::Allocating>::deallocate(pointer as *mut u8, layout)
        }
    }
}

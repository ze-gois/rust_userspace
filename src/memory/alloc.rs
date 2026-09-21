use crate::memory::heap::Allocator;
use ample::traits::Allocating;
use core::alloc::{GlobalAlloc, Layout};

struct BlergAlloc;

unsafe impl GlobalAlloc for BlergAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        <Allocator as Allocating>::allocate(layout)
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let _ = unsafe { <Allocator as Allocating>::deallocate(pointer, layout) };
    }
}

#[global_allocator]
static ALLOCATOR: BlergAlloc = BlergAlloc;

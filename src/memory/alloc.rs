use crate::memory::heap::Allocator;
use ample::traits::Allocatable;
use core::alloc::{GlobalAlloc, Layout};

struct BlergAlloc;

unsafe impl GlobalAlloc for BlergAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match Allocator::allocate(layout.size() + layout.align()) {
            core::result::Result::Ok(crate::Ok::Memory(crate::memory::Ok::HeapAllocate(m))) => unsafe {
                let m = m as *mut u8;
                m.add(layout.align())
            },
            _ => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = Allocator::deallocate(ptr as *mut Allocator, layout.size() + layout.align());
    }
}

#[global_allocator]
static ALLOCATOR: BlergAlloc = BlergAlloc;

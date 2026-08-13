pub mod entry;
pub use entry::*;

pub type List = crate::memory::stack::list::List<Entry>;

pub fn from_pointer(
    environment_pointer: crate::target::arch::Pointer,
) -> (List, crate::target::arch::Pointer) {
    let environment_pointer = environment_pointer.0 as *mut crate::target::arch::PointerType;

    let mut counter = 0usize;
    unsafe {
        while !(*environment_pointer.add(counter)).is_null() {
            counter += 1;
        }
    }

    let auxiliary_pointer =
        unsafe { (environment_pointer as crate::target::arch::PointerType).add(counter + 1) };

    let list = List::from_values(counter, |index| {
        let pointer = unsafe { *environment_pointer.add(index) };
        Entry::from_pointer(crate::target::arch::Pointer(pointer))
    });

    (list, crate::target::arch::Pointer(auxiliary_pointer))
}

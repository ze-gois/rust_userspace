pub mod entry;
pub use entry::*;

pub type List = crate::memory::stack::list::List<Entry>;

pub fn from_pointer(
    stack_pointer: crate::target::arch::Pointer,
) -> (List, crate::target::arch::Pointer) {
    let counter = unsafe { *stack_pointer.0 } as usize;
    let argument_pointers =
        unsafe { (stack_pointer.0).add(1) as *const crate::target::arch::PointerType };
    let environment_pointer = unsafe { (stack_pointer.0).add(2 + counter) };

    let list = List::from_values(counter, |index| {
        let pointer = unsafe { *argument_pointers.add(index) };
        Entry::from_pointer(crate::target::arch::Pointer(pointer))
    });

    (list, crate::target::arch::Pointer(environment_pointer))
}

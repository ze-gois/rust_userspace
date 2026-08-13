pub mod atype;
pub mod entry;

pub use atype::Type;
pub use atype::TypeTrait;
pub use entry::*;

pub type List = crate::memory::stack::list::List<Entry>;

pub fn from_pointer(
    auxiliary_pointer: crate::target::arch::Pointer,
) -> (List, crate::target::arch::Pointer) {
    let auxiliary_pointer = auxiliary_pointer.0 as *const usize;
    let mut counter = 0usize;

    loop {
        let key_pointer = unsafe { auxiliary_pointer.add(counter.saturating_mul(2)) };
        let value_pointer = unsafe { key_pointer.add(1) as *const u8 };
        let pair = Type::from_pair(key_pointer, value_pointer);
        if pair.is_null() {
            break;
        }
        counter = counter.saturating_add(1);
    }

    let latter_pointer =
        unsafe { auxiliary_pointer.add(counter.saturating_add(1).saturating_mul(2)) }
            as crate::target::arch::PointerType;

    let list = List::from_values(counter, |index| {
        let pointer = unsafe { auxiliary_pointer.add(index.saturating_mul(2)) }
            as crate::target::arch::PointerType;
        Entry::from_pointer(crate::target::arch::Pointer(pointer))
    });

    (list, crate::target::arch::Pointer(latter_pointer))
}

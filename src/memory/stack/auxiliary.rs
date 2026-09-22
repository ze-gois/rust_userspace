use ample::r#type::Vec;

pub mod entry;
pub mod r#type;

pub use entry::Entry;
pub use r#type::{Type, TypeTrait};

pub type List = Vec<Entry>;

pub unsafe fn from_pointer(auxiliary_pointer: *const usize) -> (List, *const usize) {
    let mut values = List::new();
    let mut index = 0usize;

    loop {
        let pointer = unsafe { auxiliary_pointer.add(index.saturating_mul(2)) };
        let entry = Entry::from_pointer(pointer);
        let value = entry.value();

        if value.is_null() {
            let latter = unsafe { pointer.add(2) };
            return (values, latter);
        }

        values.push(entry);
        index = index.saturating_add(1);
    }
}

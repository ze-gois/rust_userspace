use ample::r#type::Vec;

pub mod entry;
pub mod r#type;

pub use entry::Entry;
pub use r#type::{Type, TypeTrait};

#[derive(Debug, Default)]
pub struct List {
    entries: Vec<Entry>,
}

impl List {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn push(&mut self, entry: Entry) { self.entries.push(entry); }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn get(&self, index: usize) -> Option<&Entry> { self.entries.get(index) }
    pub fn iter(&self) -> core::slice::Iter<'_, Entry> { self.entries.iter() }
}

pub unsafe fn from_pointer(auxiliary_pointer: *const usize) -> (List, *const usize) {
    let mut auxiliary = List::new();
    let mut index = 0usize;

    loop {
        let pointer = unsafe { auxiliary_pointer.add(index.saturating_mul(2)) };
        let entry = Entry::from_pointer(pointer);
        let value = entry.value();

        if value.is_null() {
            let latter = unsafe { pointer.add(2) };
            return (auxiliary, latter);
        }

        auxiliary.push(entry);
        index = index.saturating_add(1);
    }
}

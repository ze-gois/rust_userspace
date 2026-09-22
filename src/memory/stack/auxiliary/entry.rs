use super::r#type::{Type, TypeTrait, Word};

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pointer: *const Word,
}

impl Entry {
    pub const fn from_pointer(pointer: *const Word) -> Self {
        Self { pointer }
    }

    pub const fn pointer(&self) -> *const Word {
        self.pointer
    }

    pub fn raw_key(&self) -> Word {
        unsafe { *self.pointer }
    }

    pub fn raw_value(&self) -> Word {
        unsafe { *self.pointer.add(1) }
    }

    pub fn value(&self) -> Type {
        Type::from_pair(self.pointer, unsafe { self.pointer.add(1) })
    }
}

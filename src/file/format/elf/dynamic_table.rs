//! ELF dynamic section resolved through section-header relations.
//!
//! The section-header `sh_link` identifies the string table used by the
//! dynamic array.

use super::{
    dynamic::{Dynamic, Tag},
    dynamic_array::DynamicArray,
    string_table::StringTable,
};

#[derive(Debug)]
pub struct DynamicTable<'file> {
    pub array: DynamicArray,
    pub strings: StringTable<'file>,
}

impl<'file> DynamicTable<'file> {
    pub const fn new(array: DynamicArray, strings: StringTable<'file>) -> Self {
        Self { array, strings }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Dynamic> {
        self.array.get(index)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Dynamic> {
        self.array.iter()
    }

    pub fn string(&self, entry: &Dynamic) -> Option<&'file str> {
        match entry.tag {
            Tag::Needed
            | Tag::SharedObjectName
            | Tag::RuntimeSearchPath
            | Tag::RunPath => self.strings.get_str(entry.payload as usize),
            _ => None,
        }
    }
}

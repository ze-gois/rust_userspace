//! ELF dynamic section resolved through section-header relations.
//!
//! The dynamic section contains a `DynamicArray`; its section header's
//! `sh_link` identifies the associated string table.

use super::{Dynamic, Tag};
use crate::file::format::elf::{
    dynamic_array::DynamicArray,
    string_table::StringTable,
};

#[derive(Debug)]
pub struct Section<'file> {
    pub array: DynamicArray,
    pub strings: StringTable<'file>,
}

impl<'file> Section<'file> {
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

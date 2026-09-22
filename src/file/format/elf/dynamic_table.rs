//! ELF dynamic section.
//!
//! The section-header `sh_link` identifies the string table used by dynamic
//! entries. The dynamic array terminates at the first `DT_NULL` entry.

use ample::r#type::Vec;

use super::{
    dynamic::{Dynamic, Tag},
    string_table::StringTable,
};

#[derive(Debug)]
pub struct DynamicTable<'file> {
    pub entries: Vec<Dynamic>,
    pub strings: StringTable<'file>,
}

impl<'file> DynamicTable<'file> {
    pub const fn new(entries: Vec<Dynamic>, strings: StringTable<'file>) -> Self {
        Self { entries, strings }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Dynamic> {
        self.entries.get(index)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Dynamic> {
        self.entries.iter()
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

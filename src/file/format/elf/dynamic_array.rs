//! ELF dynamic array (`_DYNAMIC[]`).
//!
//! The array contains `Dynamic` entries and terminates at the first
//! `DT_NULL`. Pointer-valued entries contain program virtual addresses.

use ample::r#type::Vec;

use super::dynamic::{Dynamic, Tag};

#[derive(Debug)]
pub struct DynamicArray {
    pub entries: Vec<Dynamic>,
}

impl DynamicArray {
    pub const fn new(entries: Vec<Dynamic>) -> Self {
        Self { entries }
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

    pub fn first(&self, tag: Tag) -> Option<&Dynamic> {
        self.entries.iter().find(|entry| entry.tag == tag)
    }
}

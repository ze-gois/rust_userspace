//! ELF note entries.
//!
//! Notes are variable-length records. Their fixed-size fields and padding use
//! the word size prescribed by the ELF class; the name and descriptor lengths
//! exclude padding.

use ample::r#type::Vec;

use super::note::Note;

#[derive(Debug)]
pub struct NoteTable<'file> {
    pub notes: Vec<Note<'file>>,
}

impl<'file> NoteTable<'file> {
    pub const fn new(notes: Vec<Note<'file>>) -> Self {
        Self { notes }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Note<'file>> {
        self.notes.get(index)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Note<'file>> {
        self.notes.iter()
    }
}

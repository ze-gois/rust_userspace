//! ELF note entries.
//!
//! Notes are variable-length records. Their fixed-size fields are 32-bit
//! words in both ELF classes. Name and descriptor padding follows the
//! class-specific note alignment, and their lengths exclude that padding.

use ample::r#type::Vec;

use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    OriginatorNotNullTerminated { index: usize },
}

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

    pub fn validate(&self) -> Result<(), ValidationError> {
        for (index, note) in self.notes.iter().enumerate() {
            if !note.originator.is_empty() && note.originator.last() != Some(&0) {
                return Err(ValidationError::OriginatorNotNullTerminated { index });
            }
        }

        Ok(())
    }
}

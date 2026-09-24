//! ELF string table.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    MissingInitialNull,
    MissingFinalNull,
}

#[derive(Debug, Clone, Copy)]
pub struct StringTable<'file> {
    bytes: &'file [u8],
}

impl<'file> StringTable<'file> {
    pub const fn new(bytes: &'file [u8]) -> Self {
        Self { bytes }
    }

    pub const fn bytes(&self) -> &'file [u8] {
        self.bytes
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.bytes.is_empty() {
            return Ok(());
        }

        if self.bytes.first() != Some(&0) {
            return Err(ValidationError::MissingInitialNull);
        }

        if self.bytes.last() != Some(&0) {
            return Err(ValidationError::MissingFinalNull);
        }

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&'file [u8]> {
        let tail = self.bytes.get(index..)?;
        let length = tail.iter().position(|byte| *byte == 0)?;
        tail.get(..length)
    }

    pub fn get_str(&self, index: usize) -> Option<&'file str> {
        core::str::from_utf8(self.get(index)?).ok()
    }
}

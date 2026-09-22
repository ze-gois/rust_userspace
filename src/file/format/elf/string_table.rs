//! ELF string table.

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

    pub fn get(&self, index: usize) -> Option<&'file [u8]> {
        let tail = self.bytes.get(index..)?;
        let length = tail.iter().position(|byte| *byte == 0)?;
        tail.get(..length)
    }

    pub fn get_str(&self, index: usize) -> Option<&'file str> {
        core::str::from_utf8(self.get(index)?).ok()
    }
}

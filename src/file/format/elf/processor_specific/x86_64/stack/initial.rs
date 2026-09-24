//! Initial x86-64 process stack described by the System V psABI.

use ample::r#type::Vec;

/// Required alignment of `%rsp` at process entry.
pub const ALIGNMENT: usize = 16;

/// One auxiliary-vector pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuxiliaryEntry {
    pub r#type: usize,
    pub value: usize,
}

impl AuxiliaryEntry {
    pub const NULL: Self = Self {
        r#type: 0,
        value: 0,
    };

    pub const fn new(r#type: usize, value: usize) -> Self {
        Self { r#type, value }
    }

    pub const fn is_null(self) -> bool {
        self.r#type == 0
    }
}

/// Serialized machine-word portion of the initial process stack.
///
/// Argument, environment, and auxiliary-information payloads may live in a
/// separate information block. This image stores the pointers and scalar
/// values placed at the low-address end beginning at `%rsp`.
#[derive(Debug)]
pub struct Image {
    words: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    AddressOverflow,
    InsufficientMemory {
        image_size: usize,
        memory_size: usize,
    },
}

impl Image {
    pub fn new(
        arguments: &[*const u8],
        environment: &[*const u8],
        auxiliary: &[AuxiliaryEntry],
    ) -> Self {
        let auxiliary_count = auxiliary
            .iter()
            .take_while(|entry| !entry.is_null())
            .count();

        let capacity = 1usize
            .saturating_add(arguments.len())
            .saturating_add(1)
            .saturating_add(environment.len())
            .saturating_add(1)
            .saturating_add(auxiliary_count.saturating_mul(2))
            .saturating_add(2);

        let mut words = Vec::with_capacity(capacity);

        words.push(arguments.len());
        for pointer in arguments.iter().copied() {
            words.push(pointer as usize);
        }
        words.push(0);

        for pointer in environment.iter().copied() {
            words.push(pointer as usize);
        }
        words.push(0);

        for entry in auxiliary
            .iter()
            .copied()
            .take_while(|entry| !entry.is_null())
        {
            words.push(entry.r#type);
            words.push(entry.value);
        }

        words.push(AuxiliaryEntry::NULL.r#type);
        words.push(AuxiliaryEntry::NULL.value);

        Self { words }
    }

    pub fn words(&self) -> &[usize] {
        self.words.as_slice()
    }

    pub fn word_len(&self) -> usize {
        self.words.len()
    }

    pub fn byte_len(&self) -> usize {
        self.words
            .len()
            .saturating_mul(core::mem::size_of::<usize>())
    }

    /// Write this stack image at the high-address end of `memory`.
    ///
    /// The returned pointer is the value to install in `%rsp`.
    pub fn write(&self, memory: &mut [u8]) -> Result<*mut u8, Error> {
        let memory_start = memory.as_mut_ptr() as usize;
        let memory_end = memory_start
            .checked_add(memory.len())
            .ok_or(Error::AddressOverflow)?;
        let image_size = self.byte_len();
        let unaligned = memory_end
            .checked_sub(image_size)
            .ok_or(Error::InsufficientMemory {
                image_size,
                memory_size: memory.len(),
            })?;
        let stack_pointer = align_down(unaligned, ALIGNMENT);

        if stack_pointer < memory_start {
            return Err(Error::InsufficientMemory {
                image_size,
                memory_size: memory.len(),
            });
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                self.words.as_ptr().cast::<u8>(),
                stack_pointer as *mut u8,
                image_size,
            );
        }

        Ok(stack_pointer as *mut u8)
    }
}

const fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

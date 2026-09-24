use ample::r#type::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pointer: *const u8,
}

impl Entry {
    pub const fn from_pointer(pointer: *const u8) -> Self {
        Self { pointer }
    }

    pub const fn pointer(&self) -> *const u8 {
        self.pointer
    }

    pub fn as_c_str(&self) -> Option<&core::ffi::CStr> {
        if self.pointer.is_null() {
            return None;
        }
        Some(unsafe { core::ffi::CStr::from_ptr(self.pointer.cast()) })
    }

    pub fn as_str(&self) -> Option<&str> {
        self.as_c_str()?.to_str().ok()
    }

    /// Interpret the complete argument as a decimal natural number.
    ///
    /// Only ASCII decimal digits are accepted. Signs, whitespace, empty
    /// strings, and values outside `usize` are not natural-number arguments.
    pub fn as_natural_number(&self) -> Option<usize> {
        let bytes = self.as_str()?.as_bytes();
        if bytes.is_empty() {
            return None;
        }

        let mut value = 0usize;
        for byte in bytes.iter().copied() {
            if !byte.is_ascii_digit() {
                return None;
            }

            value = value
                .checked_mul(10)?
                .checked_add(usize::from(byte - b'0'))?;
        }

        Some(value)
    }
}

#[derive(Debug, Default)]
pub struct List {
    entries: Vec<Entry>,
}

impl List {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { entries: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, entry: Entry) { self.entries.push(entry); }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn get(&self, index: usize) -> Option<&Entry> { self.entries.get(index) }
    pub fn iter(&self) -> core::slice::Iter<'_, Entry> { self.entries.iter() }
}

pub unsafe fn from_pointer(
    stack_pointer: crate::target::architecture::StackPointer,
) -> (List, *const usize) {
    let words = stack_pointer.cast::<usize>();
    let count = unsafe { *words };
    let pointers = unsafe { words.add(1) };

    let mut arguments = List::with_capacity(count);
    for index in 0..count {
        let pointer = unsafe { *pointers.add(index) } as *const u8;
        arguments.push(Entry::from_pointer(pointer));
    }

    let environment = unsafe { pointers.add(count + 1) };
    (arguments, environment)
}

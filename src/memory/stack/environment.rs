use ample::r#type::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pointer: *const u8,
}

impl Entry {
    pub const fn from_pointer(pointer: *const u8) -> Self { Self { pointer } }
    pub const fn pointer(&self) -> *const u8 { self.pointer }

    pub fn as_c_str(&self) -> Option<&core::ffi::CStr> {
        if self.pointer.is_null() {
            return None;
        }
        Some(unsafe { core::ffi::CStr::from_ptr(self.pointer.cast()) })
    }

    pub fn as_str(&self) -> Option<&str> {
        self.as_c_str()?.to_str().ok()
    }

    pub fn pair(&self) -> Option<(&str, &str)> {
        self.as_str()?.split_once('=')
    }

    pub fn key(&self) -> Option<&str> {
        self.pair().map(|(key, _)| key)
    }

    pub fn value(&self) -> Option<&str> {
        self.pair().map(|(_, value)| value)
    }

    pub fn has_separator(&self) -> bool {
        self.as_str().is_some_and(|value| value.contains('='))
    }
}

#[derive(Debug, Default)]
pub struct List {
    entries: Vec<Entry>,
}

impl List {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn push(&mut self, entry: Entry) { self.entries.push(entry); }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn get(&self, index: usize) -> Option<&Entry> { self.entries.get(index) }
    pub fn iter(&self) -> core::slice::Iter<'_, Entry> { self.entries.iter() }
}

pub unsafe fn from_pointer(environment_pointer: *const usize) -> (List, *const usize) {
    let mut environment = List::new();
    let mut index = 0usize;

    loop {
        let pointer = unsafe { *environment_pointer.add(index) };
        if pointer == 0 {
            break;
        }

        environment.push(Entry::from_pointer(pointer as *const u8));
        index += 1;
    }

    let auxiliary = unsafe { environment_pointer.add(index + 1) };
    (environment, auxiliary)
}

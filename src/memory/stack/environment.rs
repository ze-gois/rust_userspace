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

    pub fn key(&self) -> Option<&str> {
        let value = self.as_str()?;
        Some(value.split_once('=').map_or(value, |(key, _)| key))
    }

    pub fn value(&self) -> Option<&str> {
        let value = self.as_str()?;
        value.split_once('=').map(|(_, value)| value)
    }
}

pub type List = Vec<Entry>;

pub unsafe fn from_pointer(environment_pointer: *const usize) -> (List, *const usize) {
    let mut values = List::new();
    let mut index = 0usize;

    loop {
        let pointer = unsafe { *environment_pointer.add(index) };
        if pointer == 0 {
            break;
        }

        values.push(Entry::from_pointer(pointer as *const u8));
        index += 1;
    }

    let auxiliary = unsafe { environment_pointer.add(index + 1) };
    (values, auxiliary)
}

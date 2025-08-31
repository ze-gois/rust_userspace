use crate::target::arch::Pointer;

#[repr(C)]
pub struct Entry {
    pub prev: *mut Entry,
    pub next: *mut Entry,
    pub pointer: Pointer, // armazenar o ponteiro cru
}

impl Entry {
    pub fn from_pointer(pointer: Pointer) -> Entry {
        Entry {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            pointer: pointer,
        }
    }

    pub fn key(&self) -> &str {
        // Create a slice from the raw pointer
        // let s = unsafe { core::slice::from_raw_parts(self.pointer.0, l) };
        let c_str = unsafe { core::ffi::CStr::from_ptr(self.pointer.0 as *mut i8) };

        let r_str = c_str.to_str().unwrap();

        if let Some(pos) = r_str.find('=') {
            &r_str[..pos]
        } else {
            r_str
        }
    }

    pub fn value(&self) -> Option<&str> {
        let c_str = unsafe { core::ffi::CStr::from_ptr(self.pointer.0 as *mut i8) };

        let r_str = c_str.to_str().unwrap();

        // Split at '=' character to get key and value
        if let Some(pos) = r_str.find('=') {
            Some(&r_str[(pos + 1)..])
        } else {
            None
        }
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // unsafe {
        // let cstr = core::ffi::CStr::from_ptr(self.pointer.0 as *const i8);
        let _cstr = self.pointer.0;

        let _ = write!(f, "Entry: {{ ");
        let _ = write!(f, "{:?} = {:?}, ", self.key(), self.value());
        let _ = write!(f, " }}");
        return Ok(());
        // }
    }
}

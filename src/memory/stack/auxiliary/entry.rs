use super::atype;

#[repr(C)]
pub struct Entry {
    pub prev: *mut Entry,
    pub next: *mut Entry,
    pub pointer: crate::target::arch::Pointer, // armazenar o ponteiro cru
}

impl Entry {
    pub fn from_pointer(pointer: crate::target::arch::Pointer) -> Entry {
        Entry {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            pointer: pointer,
        }
    }

    pub fn key(&self) -> atype::TypeUnit {
        use atype::FromDiscriminant;
        atype::TypeUnit::from_discriminant(unsafe { *self.pointer.0 as usize })
    }

    pub fn value(&self) -> atype::Type {
        use atype::TypeTrait;
        unsafe {
            atype::Type::from_pair(
                self.pointer.0 as *mut usize,
                (self.pointer.0 as *mut usize).add(1) as *const u8,
            )
        }
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // unsafe {
        // let cstr = core::ffi::CStr::from_ptr(self.pointer.0 as *const i8);
        let _cstr = self.pointer.0;

        let _ = write!(f, "Entry: {{ ");
        let _ = write!(f, "{:?}, ", self.value());
        let _ = write!(f, " }}");
        return Ok(());
        // }
    }
}

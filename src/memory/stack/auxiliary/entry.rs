use super::atype;

ample::r#struct!(
    #[repr(C)]
    pub struct Entry {
        pub prev: *mut Entry,
        pub next: *mut Entry,
        pub pointer: crate::target::arch::Pointer, // armazenar o ponteiro cru
    }
);

impl Entry {
    pub fn from_pointer(pointer: crate::target::arch::Pointer) -> Entry {
        Entry {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            pointer: pointer,
        }
    }

    pub fn raw_key(&self) -> atype::AuxWord {
        unsafe { *(self.pointer.0 as *const atype::AuxWord) }
    }

    pub fn raw_value(&self) -> atype::AuxWord {
        unsafe { *(self.pointer.0.add(1) as *const atype::AuxWord) }
    }

    pub fn key(&self) -> atype::TypeUnit {
        use atype::FromDiscriminant;
        atype::TypeUnit::from_discriminant(self.raw_key())
    }

    pub fn value(&self) -> atype::Type {
        use atype::TypeTrait;
        unsafe {
            atype::Type::from_pair(
                self.pointer.0 as *const atype::AuxWord,
                self.pointer.0.add(1) as *const u8,
            )
        }
    }
}

impl crate::memory::stack::list::LinkedEntry for Entry {
    fn set_links(&mut self, previous: *mut Self, next: *mut Self) {
        self.prev = previous;
        self.next = next;
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // unsafe {
        // let cstr = core::ffi::CStr::from_ptr(self.pointer.0 as *const i8);
        // let _cstr = self.pointer.0;

        let _ = write!(f, "Entry: {{ ");
        let _ = write!(f, "{:?}, ", self.prev);
        let _ = write!(f, "{:?}, ", self.next);
        let _ = write!(f, "{:?}, ", self.pointer);
        let _ = write!(f, " }}");

        return Ok(());
        // }
    }
}

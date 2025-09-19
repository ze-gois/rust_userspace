ample::result!(
    Ok;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_ALLOCATE_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_MEMORY_ALLOCATE_HEAP_OK; Allocate; crate::memory::allocate::heap::Ok; "ZE"; "Entry to ze"],
    ];
    Error;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_ALLOCATE_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_MEMORY_ALLOCATE_HEAP_ERROR; Allocate; crate::memory::allocate::heap::Error; "ZE"; "Entry to ze"],
    ]
);

impl ample::traits::AllocatableResult for Ok {
    fn as_ptr(&self) -> *mut u8 {
        core::ptr::null_mut()
    }

    fn from_raw(raw: *mut u8) -> Self {
        Ok::Default(3)
    }
}

impl ample::traits::AllocatableResult for Error {
    fn as_ptr(&self) -> *mut u8 {
        core::ptr::null_mut()
    }

    fn from_raw(raw: *mut u8) -> Self {
        Error::Default(3)
    }
}

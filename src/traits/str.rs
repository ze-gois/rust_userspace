pub trait Str {
    fn open_elf(&self) -> Option<isize>;
    fn from_null_terminated_pointer(ptr: *const u8) -> Self;
    fn from_pointer(ptr: *const u8, len: usize) -> Self;
    fn from_slice(slice: &[u8]) -> Self;
    fn terminate(&mut self);
}

impl Str for &str {
    fn open_elf(&self) -> Option<isize> {
        let (is_it, file_descriptor) =
            crate::file::format::elf::header::Identifier::is_file_path_magical(self);

        if is_it {
            Some(file_descriptor)
        } else {
            let _ = crate::target::os::syscall::close(file_descriptor);
            None
        }
    }

    fn from_null_terminated_pointer(ptr: *const u8) -> Self {
        let cstr = unsafe { core::ffi::CStr::from_ptr(ptr as *mut i8) };
        cstr.to_str().unwrap()
    }

    fn from_pointer(ptr: *const u8, len: usize) -> Self {
        let cstr = unsafe { core::ffi::CStr::from_ptr(ptr as *mut i8) };
        cstr.to_str().unwrap()
    }

    fn from_slice(slice: &[u8]) -> Self {
        Self::from_pointer(slice.as_ptr(), slice.len())
    }

    fn terminate(&mut self) {
        // match ample::string::terminate::<crate::Origin, crate::Origin, crate::memory::heap::Allocator>(
        //     &self,
        // ) {
        //     Ok(crate::Ok(_)) => {}
        //     Err(crate::Error(_)) => {}
        // }
    }
}

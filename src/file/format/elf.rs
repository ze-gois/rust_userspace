pub mod dtype;

pub mod header;
pub use header::Header32;
pub use header::Header64;

pub mod section;
pub mod segment;

pub mod transfer;

pub fn execute_from_path(
    path: &str,
    stack_pointer: crate::target::arch::PointerType,
) -> core::result::Result<(), crate::file::format::elf::segment::Error> {
    let (is_elf, file_descriptor) =
        crate::file::format::elf::header::Identifier::is_file_path_magical(path);
    if file_descriptor >= 0 {
        let _ = crate::target::os::syscall::close(file_descriptor);
    }
    if !is_elf {
        return Err(crate::file::format::elf::segment::Error::InvalidHeader);
    }

    let prepared = match crate::file::format::elf::segment::prepare_execution(
        path,
        path.as_ptr(),
        stack_pointer,
    ) {
        Ok(prepared) => prepared,
        Err(error) => return Err(error),
    };

    let new_stack = crate::memory::Stack::from_pointer(crate::target::arch::Pointer(stack_pointer));
    new_stack.print();

    unsafe {
        crate::file::format::elf::transfer::jump_to_entry(prepared.entry, prepared.stack_pointer)
    }
}

pub mod result;
pub use result::{Error, Ok, Result};

// pub fn execute_from_path(path: &str) -> Result<!, Error> {
//     // let (header, file_descriptor) = header::Identifier::from_path(path)?;

// }

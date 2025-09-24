#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use userspace;
use userspace::file::traits::Readable;
use userspace::info;

#[derive(Debug)]
pub struct Origin;

ample::trait_implement_primitives!();

#[rustfmt::skip]
#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: userspace::target::arch::PointerType) -> ! {
    let stack_pointer = userspace::target::arch::Pointer(stack_pointer);

    info!("eXecuting Executable and Linkable Format\n\n\n");

    let argc = stack_pointer.0 as *const usize;
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    // stack.print();
    stack.arguments.print();

    let self_file_path_pointer = stack.arguments.get(0).unwrap();

    if !self_file_path_pointer.pointer.0.is_null() {
        unsafe {
            let cstr = core::ffi::CStr::from_ptr(self_file_path_pointer.pointer.0 as *mut i8);
            let self_path = cstr.to_str().unwrap();

            // let (fd, stat, fptr) = userspace::file::load(self_path).unwrap();
            let (identifier, _file_descriptor, _offset) = userspace::file::format::elf::header::Identifier::read_from_file_path(self_path, 0, true);

            info!("identifier={}\n\n", identifier);
            //
            let (magic, file_descriptor) = userspace::file::format::elf::header::identifier::Magic::read_from_file_path_offsets(self_path, &[0,1,2,3], true);

            info!("\n\n{:?}\n\n{:?}", magic, file_descriptor);

            let (magic, file_descriptor, offset) = <[userspace::file::format::elf::dtype::class_64::UChar;4]>::read_from_file_path(self_path, 0, true);
            info!("\n\n{:?}\n{:?}\n{:?}", magic, file_descriptor, offset);

        }
    }


    // let uchar32 = userspace::file::format::elf::dtype::class_32::UChar(3);

    panic!();
}

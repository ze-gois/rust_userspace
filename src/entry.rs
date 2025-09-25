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
        use userspace::traits::Str;

        let self_path = <&str>::from_null_terminated_pointer(self_file_path_pointer.pointer.0 as *const u8);

        let Some(self_elf_file_descriptor) = <&str>::open_elf(&self_path) else {
            userspace::target::os::syscall::exit(32)
        };

        let (self_elf_identifier,current_offset) = userspace::file::format::elf::header::Identifier::read_from_file_descriptor(self_elf_file_descriptor, 0, true);

        info!("\n{:?}\n",self_elf_identifier);

        let endianness = match self_elf_identifier.data() {
            userspace::file::format::elf::header::identifier::Data::DataLSB => true,
            userspace::file::format::elf::header::identifier::Data::DataMSB => false,
            userspace::file::format::elf::header::identifier::Data::DataNone => userspace::target::os::syscall::exit(33),
        };

        let (header, current_offset) = userspace::file::format::elf::Header64::read_from_file_descriptor(self_elf_file_descriptor, 0, endianness);

        info!("\n{:?}\n",header);

        extern crate alloc;

        let x = alloc::string::String::new();

        userspace::info!("\n\n=>>>{:?}\n\n",x);

    }


    // let uchar32 = userspace::file::format::elf::dtype::class_32::UChar(3);

    panic!();
}

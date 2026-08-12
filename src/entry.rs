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

        let (self_elf_identifier, _) = userspace::file::format::elf::header::Identifier::read_from_file_descriptor(self_elf_file_descriptor, 0, true);

        info!("\n{:?}\n",self_elf_identifier);

        let endianness = match self_elf_identifier.data() {
            userspace::file::format::elf::header::identifier::Data::DataLSB => true,
            userspace::file::format::elf::header::identifier::Data::DataMSB => false,
            userspace::file::format::elf::header::identifier::Data::DataNone => userspace::target::os::syscall::exit(33),
        };

        let (header, _) = userspace::file::format::elf::Header64::read_from_file_descriptor(self_elf_file_descriptor, 0, endianness);

        info!("\n{:?}\n",header);

        extern crate alloc;

        let x = alloc::string::String::new();

        userspace::info!("\n\n=>>>{:?}\n\n",x);

    }


    // Replace this process with `/usr/bin/ls`.
    // The argument and environment vectors are terminated by null pointers,
    // as required by Linux's execve(2) ABI.
    let ls_path = b"/usr/bin/ls\0";
    let ls_name = b"ls\0";
    let ls_arguments = [ls_name.as_ptr(), core::ptr::null()];
    let empty_environment = [core::ptr::null()];

    let execve_result = userspace::target::os::syscall::execve(
        ls_path.as_ptr(),
        ls_arguments.as_ptr(),
        empty_environment.as_ptr(),
    );

    info!("execve /usr/bin/ls failed: {:?}\n", execve_result);
    userspace::target::os::syscall::exit(127);
}

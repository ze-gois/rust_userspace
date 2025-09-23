#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ample::traits::Bytes;
use userspace;
use userspace::info;
use userspace::memory::heap::Allocating;
use userspace::target;

#[derive(Debug)]
pub struct Origin;

ample::trait_implement_primitives!();

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: crate::target::arch::PointerType) -> ! {
    let stack_pointer = crate::target::arch::Pointer(stack_pointer);

    info!("eXecuting Executable and Linkable Format\n\n\n");

    let argc = stack_pointer.0 as *const usize;
    info!("argc={:?}\n\n", unsafe { *argc });
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    stack.print();
    stack.arguments.print();

    let arg0 = stack.arguments.get(0).unwrap();

    if !arg0.pointer.0.is_null() {
        unsafe {
            let cstr = core::ffi::CStr::from_ptr(arg0.pointer.0 as *mut i8);
            let self_path = cstr.to_str().unwrap();

            userspace::info!("\n{:?}\n\n", self_path);

            let self_fd = userspace::file::open(self_path);

            let (fd, stat, ptr) = userspace::file::load(self_path).unwrap();

            info!("fd={:?}\n\n stat={:?}\n\n ptr={:?}\n\n", fd, stat, ptr);

            for c in 0..=15 {
                info!("*ptr.add({:?}) as char == {:?}\n", c, *ptr.add(c) as char);
            }

            let entries = userspace::memory::stack::auxiliary::Entry::allocate_slice(10);

            for e in entries.iter_mut() {
                info!("{:?}\n", e);
            }

            use userspace::file::traits::Readable;

            let (identifier, offset) =
                userspace::file::format::elf::header::Identifier::read_from_pointer(ptr, 0, true);

            userspace::info!("{:?}\n\n", identifier);

            let (identifier, offset) =
                userspace::file::format::elf::header::Identifier::read_from_file_path(
                    self_path, 0, true,
                );

            userspace::info!("{:?}\n\n", identifier);

            userspace::info!(
                "{:?}\n",
                userspace::file::format::elf::header::Identifier::BYTES_SIZE
            );

            let fil = <[userspace::file::format::elf::header::Identifier; 1] as Readable<
                userspace::Origin,
            >>::read_from_file_path(self_path, 0, true);
            userspace::info!("File content: {:?}", fil);

            let magic = u8::read_from_file_path_offsets(self_path, &[0, 1, 2, 3], true);
            userspace::info!("\n\n{:?}\n", magic);

            // let magic = u8::read_from_file_path_offsets(self_path, 0..4, true);
            // userspace::info!("\n\n{:?}\n", magic);
        }
    }

    // let uchar32 = userspace::file::format::elf::dtype::class_32::UChar(3);

    panic!();
}

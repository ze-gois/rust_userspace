#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_imports)]

use userspace;
use userspace::info;
use userspace::target;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: crate::target::arch::PointerType) -> ! {
    let stack_pointer = crate::target::arch::Pointer(stack_pointer);

    info!("eXecuting Executable and Linkable Format\n\n");

    let argc = stack_pointer.0 as *const usize;
    info!("argc={:?}\n", unsafe { *argc });
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    // stack.print();
    stack.arguments.print();

    let arg0 = stack.arguments.get(0).unwrap();
    let arg0_pointer = arg0.pointer;

    if !arg0.pointer.0.is_null() {
        unsafe {
            let cstr = core::ffi::CStr::from_ptr(arg0.pointer.0 as *mut i8);
            let self_path = cstr.to_str().unwrap();
            userspace::info!("\n{:?}\n", self_path);
            let identifier = userspace::file::format::elf::header::Identifier::from_path(self_path);
            userspace::info!("{:?}\n", identifier);
        }
    }

    let uchar32 = userspace::file::format::elf::dtype::class_32::UChar(3);

    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn flag_license(stack_pointer: crate::target::arch::PointerType) -> () {
    crate::info!("It is goooood fo you\n");
    crate::file::print("LICENSE");
    unsafe {
        core::arch::asm!("mov {0}, rdi","call entry", in(reg) stack_pointer);
    }
}

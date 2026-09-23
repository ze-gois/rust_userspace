//! x86-64 ELF entry-point transfer semantics.
//!
//! The callable helper proves executable mappings independently. Process
//! transfer installs the psABI entry state we own and does not return.

pub type Callable = unsafe extern "C" fn() -> usize;

/// Function pointer supplied in `%rdx` at process entry.
pub type TerminationFunction = unsafe extern "C" fn();

/// Call executable code at `address` and return its `RAX` result.
///
/// # Safety
///
/// `address` must point to executable x86-64 code that obeys the active
/// System V calling convention and returns normally.
pub unsafe fn call(address: *const u8) -> usize {
    let callable: Callable = unsafe { core::mem::transmute(address) };
    unsafe { callable() }
}

/// Transfer control to an ELF process entry point.
///
/// This installs the initial stack pointer in `%rsp`, places the optional
/// termination function in `%rdx`, marks the outermost frame with zero
/// `%rbp`, establishes the psABI arithmetic/direction flag state, and jumps
/// to `entry_point`.
///
/// # Safety
///
/// `entry_point` must point to executable x86-64 process-entry code.
/// `stack_pointer` must point to a valid x86-64 initial process stack and be
/// 16-byte aligned. The pointed-to stack and all referenced information must
/// remain valid for the lifetime of the transferred process.
pub unsafe fn transfer_control(
    entry_point: *const u8,
    stack_pointer: *mut u8,
    termination_function: Option<TerminationFunction>,
) -> ! {
    let termination_address =
        termination_function.map_or(0usize, |function| function as usize);

    unsafe {
        core::arch::asm!(
            "mov rsp, rsi",
            "xor ebp, ebp",
            "mov eax, 1",
            "add eax, 1",
            "cld",
            "jmp rdi",
            in("rdi") entry_point,
            in("rsi") stack_pointer,
            in("rdx") termination_address,
            options(noreturn),
        )
    }
}

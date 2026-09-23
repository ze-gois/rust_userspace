//! x86-64 callable entry-point invocation.
//!
//! This is a deliberately narrow execution primitive used to prove that code
//! copied from an ELF object into an executable mapping can be entered and can
//! return through the x86-64 System V function-call convention.
//!
//! It is not the final ELF process-entry handoff. A real process entry is
//! `noreturn` from the loader's perspective and requires the process stack and
//! other psABI-defined initial state to be established first.

pub type Callable = unsafe extern "C" fn() -> usize;

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

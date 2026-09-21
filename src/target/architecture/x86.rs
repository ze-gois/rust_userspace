/// 64-bit x86 implementation.
///
/// Rust names this target architecture `x86_64`; the implementation is kept
/// under the x86 family rather than encoding hierarchy with an underscore.
#[cfg(target_arch = "x86_64")]
pub mod bit64;

pub use crate::target::architecture::page::BASE_BASE_SIZE;

const MASK: usize = !(BASE_SIZE - 1);

#[inline]
pub const fn align_down(address: usize) -> usize {
    address & MASK
}

#[inline]
pub const fn align_up(address: usize) -> Option<usize> {
    match address.checked_add(BASE_SIZE - 1) {
        Some(address) => Some(align_down(address)),
        None => None,
    }
}

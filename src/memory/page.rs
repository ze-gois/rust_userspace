use core::ops::Not;

pub use crate::target::arch::page::SIZE;

pub const ALIGNMENT: usize = SIZE - 1;
pub const MASK: usize = !ALIGNMENT;

pub const DYNAMIC_OFFSET: usize = 0x400000;

pub fn round_address_to_lower_page_boundary(address: usize) -> usize {
    address & MASK
}

pub fn align_to_lower_page(address: usize) -> usize {
    (address + ALIGNMENT) & ALIGNMENT.not()
}

pub fn truncate_to_page(address: usize) -> usize {
    address & ALIGNMENT.not()
}

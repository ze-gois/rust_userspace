//! ELF base address used during execution.
//!
//! The gABI computes the base address from the actual memory load address,
//! the maximum page size, and the lowest `PT_LOAD.p_vaddr`. The object file
//! supplies the latter; the execution environment supplies the former two.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseAddress(u64);

impl BaseAddress {
    pub fn calculate(
        memory_load_address: u64,
        lowest_load_virtual_address: u64,
        maximum_page_size: u64,
    ) -> Option<Self> {
        if maximum_page_size == 0 || !maximum_page_size.is_power_of_two() {
            return None;
        }

        let mask = maximum_page_size.checked_sub(1)?;
        let memory = memory_load_address & !mask;
        let object = lowest_load_virtual_address & !mask;

        memory.checked_sub(object).map(Self)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn relocate_virtual_address(
        self,
        link_time_virtual_address: u64,
    ) -> Option<u64> {
        link_time_virtual_address.checked_add(self.0)
    }
}

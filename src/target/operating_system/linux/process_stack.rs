//! Linux mapping used to hold an ELF initial process stack.

use crate::file::format::elf::processor_specific::x86_64::stack::initial;

use super::syscall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidSize { size: usize },
    MappingFailed,
    InitialStack(initial::Error),
}

#[derive(Debug)]
pub struct Mapping {
    address: *mut u8,
    length: usize,
}

impl Mapping {
    pub const fn address(&self) -> *mut u8 {
        self.address
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub fn write(
        &mut self,
        initial_stack: &initial::Image,
    ) -> Result<*mut u8, Error> {
        let memory = unsafe {
            core::slice::from_raw_parts_mut(self.address, self.length)
        };

        initial_stack
            .write(memory)
            .map_err(Error::InitialStack)
    }

    pub fn unmap(self) -> bool {
        syscall::munmap(self.address, self.length).is_ok()
    }
}

pub fn map(size: usize) -> Result<Mapping, Error> {
    if size == 0 {
        return Err(Error::InvalidSize { size });
    }

    let protection =
        (syscall::mmap::Protection::READ | syscall::mmap::Protection::WRITE)
            .bits();
    let flags = (
        syscall::mmap::Flag::PRIVATE
            | syscall::mmap::Flag::ANONYMOUS
            | syscall::mmap::Flag::STACK
    )
    .bits() as i32;

    let address = mmap_address(syscall::mmap(
        core::ptr::null_mut(),
        size,
        protection,
        flags,
        -1,
        0,
    ))
    .ok_or(Error::MappingFailed)?;

    Ok(Mapping {
        address: address as *mut u8,
        length: size,
    })
}

fn mmap_address(result: crate::Result) -> Option<usize> {
    match result {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::operating_system::Ok::Syscall(
                    crate::target::operating_system::syscall::Ok::MMap(
                        crate::target::operating_system::syscall::mmap::Ok::Default(address),
                    ),
                ),
            ),
        )) => Some(address),
        _ => None,
    }
}

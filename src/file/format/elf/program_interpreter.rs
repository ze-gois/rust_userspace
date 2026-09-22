//! ELF program interpreter.
//!
//! A `PT_INTERP` entry identifies a null-terminated pathname contained in
//! the object file.

use core::ffi::CStr;

use super::program_header::ProgramHeader;

#[derive(Debug, Clone, Copy)]
pub struct ProgramInterpreter<'file> {
    pub program_header: ProgramHeader,
    pathname: &'file CStr,
}

impl<'file> ProgramInterpreter<'file> {
    pub const fn new(program_header: ProgramHeader, pathname: &'file CStr) -> Self {
        Self {
            program_header,
            pathname,
        }
    }

    pub const fn pathname(&self) -> &'file CStr {
        self.pathname
    }

    pub fn pathname_str(&self) -> Option<&'file str> {
        self.pathname.to_str().ok()
    }
}

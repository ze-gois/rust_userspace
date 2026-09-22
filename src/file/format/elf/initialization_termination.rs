//! Initialization and termination functions described by the ELF dynamic array.
//!
//! This module describes what the object file requests. Calling functions,
//! dependency ordering across objects, and psABI interpretation of function
//! pointers belong to consumers of the ELF format.

use ample::r#type::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionAddress(u64);

impl FunctionAddress {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionPointer(u64);

impl FunctionPointer {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct Initialization {
    pub function: Option<FunctionAddress>,
    pub functions: Vec<FunctionPointer>,
}

impl Initialization {
    pub const fn new(
        function: Option<FunctionAddress>,
        functions: Vec<FunctionPointer>,
    ) -> Self {
        Self {
            function,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct PreInitialization {
    pub functions: Vec<FunctionPointer>,
}

impl PreInitialization {
    pub const fn new(functions: Vec<FunctionPointer>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct Termination {
    pub functions: Vec<FunctionPointer>,
    pub function: Option<FunctionAddress>,
}

impl Termination {
    pub const fn new(
        functions: Vec<FunctionPointer>,
        function: Option<FunctionAddress>,
    ) -> Self {
        Self {
            functions,
            function,
        }
    }

    pub fn reverse_functions(
        &self,
    ) -> core::iter::Rev<core::slice::Iter<'_, FunctionPointer>> {
        self.functions.iter().rev()
    }
}

#[derive(Debug)]
pub struct Functions {
    pub pre_initialization: Option<PreInitialization>,
    pub initialization: Initialization,
    pub termination: Termination,
}

impl Functions {
    pub const fn new(
        pre_initialization: Option<PreInitialization>,
        initialization: Initialization,
        termination: Termination,
    ) -> Self {
        Self {
            pre_initialization,
            initialization,
            termination,
        }
    }
}

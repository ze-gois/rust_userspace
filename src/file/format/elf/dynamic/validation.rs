//! gABI validation for dynamic-array relationships.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InitializationArrayMissingSize,
    TerminationArrayMissingSize,
    PreInitializationArrayMissingSize,
    PreInitializationInSharedObject,
    InitializationArraySizeNotPointerMultiple,
    TerminationArraySizeNotPointerMultiple,
    PreInitializationArraySizeNotPointerMultiple,
}

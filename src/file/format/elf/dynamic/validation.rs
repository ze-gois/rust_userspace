//! gABI validation for dynamic-array relationships.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InitializationArrayMissingSize,
    InitializationArraySizeWithoutArray,
    TerminationArrayMissingSize,
    TerminationArraySizeWithoutArray,
    PreInitializationArrayMissingSize,
    PreInitializationArraySizeWithoutArray,
    PreInitializationInSharedObject,
    InitializationArraySizeNotPointerMultiple,
    TerminationArraySizeNotPointerMultiple,
    PreInitializationArraySizeNotPointerMultiple,
}

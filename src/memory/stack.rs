pub mod arguments;
pub mod auxiliary;
pub mod environment;
pub mod region;

pub use region::{Growth, Region};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Raw,
    Modified,
}

/// Linux initial process stack reconstructed from the stack pointer supplied
/// by `_start`.
///
/// The pointed-to strings and auxiliary data remain owned by the original
/// process stack. This structure owns only the vectors of descriptors used to
/// navigate that layout.
#[derive(Debug)]
pub struct Stack {
    pub former: crate::target::architecture::StackPointer,
    pub latter: *const u8,
    pub arguments: arguments::List,
    pub environment: environment::List,
    pub auxiliary: auxiliary::List,
    pub status: Status,
}

impl Stack {
    /// Reconstruct the Linux initial process stack from the untouched pointer
    /// received from `start.s`.
    ///
    /// # Safety
    ///
    /// `stack_pointer` must point to a valid Linux initial process stack:
    /// argc, argv pointers terminated by null, envp pointers terminated by
    /// null, followed by an auxiliary vector terminated by AT_NULL.
    pub unsafe fn from_pointer(
        stack_pointer: crate::target::architecture::StackPointer,
    ) -> Self {
        let (arguments, environment_pointer) =
            unsafe { arguments::from_pointer(stack_pointer) };
        let (environment, auxiliary_pointer) =
            unsafe { environment::from_pointer(environment_pointer) };
        let (auxiliary, latter_pointer) =
            unsafe { auxiliary::from_pointer(auxiliary_pointer) };

        Self {
            former: stack_pointer,
            latter: latter_pointer.cast::<u8>(),
            arguments,
            environment,
            auxiliary,
            status: Status::Raw,
        }
    }

    pub fn argc(&self) -> usize {
        self.arguments.len()
    }
}

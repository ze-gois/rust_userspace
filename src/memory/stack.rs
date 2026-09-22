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

    pub fn print(&self) {
        crate::info!("--- Linux Initial Stack ---\n");
        crate::info!(
            "former={:p} latter={:p} argc={} status={:?}\n",
            self.former,
            self.latter,
            self.argc(),
            self.status,
        );

        crate::info!("argv[{}]\n", self.arguments.len());
        for (index, argument) in self.arguments.iter().enumerate() {
            match argument.as_str() {
                Some(value) => crate::info!("  argv[{}] = {:?}\n", index, value),
                None => crate::info!(
                    "  argv[{}] = <non-utf8 @ {:p}>\n",
                    index,
                    argument.pointer(),
                ),
            }
        }

        crate::info!("envp[{}]\n", self.environment.len());
        for (index, variable) in self.environment.iter().enumerate() {
            match variable.pair() {
                Some((key, value)) => crate::info!(
                    "  envp[{}] = {{ key: {:?}, value: {:?} }}\n",
                    index,
                    key,
                    value,
                ),
                None => match variable.as_str() {
                    Some(raw) => crate::info!(
                        "  envp[{}] = {{ raw: {:?}, separator: false }}\n",
                        index,
                        raw,
                    ),
                    None => crate::info!(
                        "  envp[{}] = <non-utf8 @ {:p}>\n",
                        index,
                        variable.pointer(),
                    ),
                },
            }
        }

        crate::info!("auxv[{}]\n", self.auxiliary.len());
        for (index, auxiliary) in self.auxiliary.iter().enumerate() {
            crate::info!(
                "  auxv[{}] = {{ type: {}, raw: {:#x}, value: {:?} }}\n",
                index,
                auxiliary.raw_key(),
                auxiliary.raw_value(),
                auxiliary.value(),
            );
        }

        crate::info!("---------------------------\n");
    }
}

pub use crate::info;
pub mod arguments;
pub mod auxiliary;
pub mod build;
pub mod constants;
pub mod environment;
pub mod list;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidSource,
    StackConstructionFailed,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Raw,
    Modfied,
}

ample::r#struct!(
    #[derive(Debug)]
    pub struct ArgumentNode {
        pub pointer: crate::target::arch::PointerType,
    }
);

ample::enum_typed!(
    u32;
    #[derive(Debug)]
    pub enum StackNode {
        A(ArgumentNode) = 1,
        B(()) = 2,
    }
);

#[repr(C)]
#[derive(Debug)]
pub struct Stack {
    pub former: crate::target::arch::Pointer,
    pub latter: crate::target::arch::Pointer,
    pub arguments: arguments::List,
    pub environment: environment::List,
    pub auxiliary: auxiliary::List,
    pub status: Status,
}

impl Stack {
    pub fn from_pointer(stack_pointer: crate::target::arch::Pointer) -> Self {
        let (arguments, environment_pointer) = arguments::from_pointer(stack_pointer);
        let (environment, auxiliary_pointer) = environment::from_pointer(environment_pointer);
        let (auxiliary, latter_pointer) = auxiliary::from_pointer(auxiliary_pointer);
        Self {
            former: stack_pointer,
            latter: latter_pointer,
            arguments,
            environment,
            auxiliary,
            status: Status::Raw,
        }
    }

    pub fn current() -> Self {
        Self::from_pointer(crate::target::arch::Pointer::current())
    }

    pub fn build_execution_stack(
        initial_stack: crate::target::arch::PointerType,
        path: &str,
        path_pointer: *const u8,
        entry: u64,
        phdr: u64,
        phent: usize,
        phnum: usize,
        interpreter_base: usize,
    ) -> Result<crate::target::arch::PointerType, Error> {
        build::build_initial_stack(
            initial_stack,
            path,
            path_pointer,
            entry,
            phdr,
            phent,
            phnum,
            interpreter_base,
        )
    }

    pub fn print(&self) {
        info!("--- Stack Contents ---\n");
        info!(
            "pub struct Stack {{
                pub former: crate::target::arch::Pointer = {:?},
                pub latter: crate::target::arch::Pointer = {:?},
                pub arguments: arguments::List = {:?},
                pub status: Status = {:?},
            }}\n",
            self.former, self.latter, self.arguments, self.status,
        );
        self.arguments.print();
        self.environment.print();
        self.auxiliary.print();
        info!("---------------------\n");
    }
}

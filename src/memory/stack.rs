pub use crate::info;
pub mod arguments;
pub mod auxiliary;
pub mod environment;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Raw,
    Modfied,
}

#[repr(C)]
#[derive(Debug)]
pub struct Stack {
    pub former: crate::target::arch::Pointer,
    pub latter: crate::target::arch::Pointer,
    // pub size: usize,
    // pub size_modified: usize,
    pub arguments: arguments::List,
    pub environment: environment::List,
    pub auxiliary: auxiliary::List,
    pub status: Status,
}

impl Stack {
    pub fn from_pointer(stack_pointer: crate::target::arch::Pointer) -> Self {
        let (arguments, environment_pointer) = arguments::List::from_pointer(stack_pointer);
        let (environment, auxiliary_pointer) = environment::List::from_pointer(environment_pointer);
        let (auxiliary, latter_pointer) = auxiliary::List::from_pointer(auxiliary_pointer);
        // let latter_pointer = auxiliary_pointer;
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

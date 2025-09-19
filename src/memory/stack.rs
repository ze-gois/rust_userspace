pub use crate::info;
pub mod arguments;
pub mod auxiliary;
pub mod environment;

// impl<A: ample::traits::Bytes<ample::Origin>> ample::traits::Bytes<crate::Origin> for A {
//     const BYTES_ALIGN: usize = A::BYTES_ALIGN;
//     const BYTES_SIZE: usize = A::BYTES_SIZE;
//     fn from_bytes(
//         bytes: [u8; <Self as ample::traits::Bytes<crate::Origin>>::BYTES_SIZE],
//         endianness: bool,
//     ) -> Self
//     where
//         Self: Sized,
//         [u8; <Self as ample::traits::Bytes<crate::Origin>>::BYTES_SIZE]:,
//     {
//         let crate_bytes = [0u8; <Self as ample::traits::Bytes<crate::Origin>>::BYTES_SIZE];
//         crate_bytes.copy_from_slice(<A as ample::traits::Bytes<ample::Origin>>::to_bytes(
//             &self, endianness,
//         ));
//     }
// }

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
        // pub string: ample::String,
    }
);

ample::r#enum!(
    u32;
    #[derive(Debug, Clone, Copy)]
    pub enum StackNode {
        A(ArgumentNode) = 1,
        B(()) = 2,
    }
);

// pub type Stack = ample::list::LinkedList<crate::Origin,crate::memory::Origin, >

#[repr(C)]
#[derive(Debug)]
pub struct Stack {
    // pub meta: crate::target::arch::Pointer,
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

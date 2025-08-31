#[macro_export]
#[rustfmt::skip]
macro_rules! error_typed {
    (
        $label:expr;
        $($module:tt)::*;
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $type:ty;
                    $argumento:ident;
                    {$($lambda_from_franco:tt)*};
                    {$($lambda_to_franco:tt)*};
                    $const_identifier:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        pub type Franco = *const u8;

        pub use $($module)::*::*;

        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $const_identifier: usize = $discriminant;
            )*
        }

        pub mod types {
            pub use $($module)::*::*;
            $(
                pub type $identifier = $type;
            )*
        }

        #[derive(Debug, Clone, Copy)]
        pub enum Error {
            $($identifier($type),)*
            TODO,
        }

        impl Error {
            pub fn description(&self) -> &str {
                match *self {
                    $( Error::$identifier(_) => $description,)*
                    Error::TODO => "Error::TODO",
                }
            }

            pub fn acronym(&self) -> &str {
                match *self {
                    $( Error::$identifier(_) => $acronym,)*
                    Error::TODO => "Error::TODO",
                }
            }

            pub fn discriminant(&self) -> usize {
                match *self {
                    $( Error::$identifier(_) => $discriminant,)*
                    Error::TODO => usize::MAX,
                }
            }


            pub fn from_ptr(value_pointer: Franco) -> Self {
                let discriminant_pointer = value_pointer as *mut usize;
                let discriminant = unsafe { *discriminant_pointer };
                let payload_pointer = unsafe { discriminant_pointer.add(1) as Franco };
                match discriminant {
                    $( $discriminant => Error::$identifier(unsafe { (|$argumento:Franco|{ $($lambda_from_franco)* })(payload_pointer) }),)*
                    _ => Error::TODO,
                }
            }

            pub fn as_ptr(&self) -> Franco {
                core::ptr::null_mut()
                // match self {
                //     $(
                //         Error::$identifier(payload) => {
                //             let discriminant_size = core::mem::size_of::<usize>();
                //             let payload_size = core::mem::size_of::<usize>();

                //             let payload_as_franco =  (|$argumento:$type|{ $($lambda_to_franco)* })(payload);
                //             let discriminant_as_franco = self.discriminant() as Franco;

                //             let pointer : &[u8] = &[0u8; discriminant_size + payload_size];
                //         },
                //     )*
                //     Error::TODO => {
                //         let discriminant_size = core::mem::size_of::<usize>();
                //         let payload_size = core::mem::size_of::<usize>();

                //         let pointer : &[u8] = &[0u8; discriminant_size + payload_size];

                //         ptr::
                //         Error::TODO
                //     }
                // }

                // match discriminant {
                //     $(
                //         $discriminant => {
                //             let discriminant_size = core::mem::size_of::<usize>();
                //             let payload_size = core::mem::size_of::<$type>();

                //             let pointer : &[u8] = &[0u8; discriminant_size + payload_size];

                //             Error::$identifier(value)
                //             // let payload_pointer = unsafe { discriminant_pointer.add(1) as Franco };
                //             // (discriminant, payload_pointer)
                //         }
                //     )*
                // }
                // match *self {
                //     $(  => (self.discriminant(), (,)*
                //     Error::TODO => (Error::TODO.discriminant(), core::ptr::null_mut()),
                // }
            }


        }
    };
}
pub use error_typed;

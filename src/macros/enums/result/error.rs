/// A macro that defines an error type and error handling for syscalls.
///
/// This macro generates:
/// 1. An Error enum with the specified variants and their associated errno values
/// 2. ErrorTrait implementation for the Error type with proper errno mapping
/// 3. An errno module with standard Linux error constants
/// 4. Into<isize> implementation for the Error type
/// 5. A handle_result function that maps arch errors to syscall errors
///
/// # Arguments
///
/// * `$error_enum_name` - The name of the error enum (usually Error)
/// * `$result_variant` - The variant name in the crate::result::Error enum (e.g., Open, Read, Write)
/// * `$syscall_name` - String slice with the syscall name
/// * A list of error variants with their descriptions, errno values and Linux standard constant names
///   [VariantName, errno_value, "description", "LINUX_CONSTANT"]
///
/// # Example
///
/// ```
/// define_error!(Error, Read, "read", [
///     [BadFileDescriptor, -9, "Bad file descriptor", "EBADF"],
///     [InvalidBuffer, -14, "Invalid buffer pointer", "EFAULT"]
/// ]);
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! define_error{
    (
        $label:expr,
        [
            $(
                [
                    $variant_discriminant:expr;
                    $variant_identifier:ident;
                    $variant_constant:ident;
                    $variant_acronym:expr;
                    $variant_descriptor:expr
                ]
            ),*
            $(,)?
        ]
    ) => {
        use result::ErrorTrait;

        pub type discriminant_type = usize;

        pub mod constant {
            pub const LABEL : &str = $label;

            $(
                pub const $variant_constant : usize = $variant_discriminant;
            )*
        }

        #[repr(usize)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $($variant_identifier = $variant_discriminant,)*
            TODO = <usize>::MAX,
        }

        impl ErrorTrait for Error {
            fn from_no(discriminant: usize) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier,)*
                    _ => Self::TODO,
                }
            }

            fn to_no(&self) -> usize {
                match *self {
                    $(Self::$variant_identifier => $variant_discriminant,)*
                    _ => <usize>::MAX,
                }
            }

            fn description(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier => $variant_descriptor,)*
                    _ => "TODO",
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier => $variant_acronym,)*
                    _ => "TODO",
                }
            }

            fn from_ptr(ptr: *const u8) -> Self {
                Self::from_no(unsafe{*(ptr as *const usize)})
            }

            fn as_ptr(ptr: Self) -> *const u8 {
                ptr.to_no() as *const u8
            }
        }

        impl Into<usize> for Error {
            fn into(self) -> usize {
                self.to_no()
            }
        }

        impl Into<isize> for Error {
            fn into(self) -> isize {
                self.to_no() as isize
            }
        }

        impl Default for Error {
            fn default() -> Self {
                Self::TODO
            }
        }
        pub type Result = core::result::Result<usize, Error>;
    };
}
pub use define_error;

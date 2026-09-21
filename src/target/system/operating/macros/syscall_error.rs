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
/// define_syscall_error!(Error, Read, "read", [
///     [BadFileDescriptor, -9, "Bad file descriptor", "EBADF"],
///     [InvalidBuffer, -14, "Invalid buffer pointer", "EFAULT"]
/// ]);
/// ```
#[macro_export]
macro_rules! define_syscall_error {
    ($error_enum_name:ident, $result_variant:ident, $syscall_name:expr,
     [ $( [$error_variant:ident, $errno:expr, $description:expr, $linux_const:ident] ),* $(,)? ]) => {
        use result::ErrorTrait;

        // Syscall name as a static string
        static SYSCALL_NAME: &'static str = $syscall_name;

        // Define Linux standard error constants in an errno module with standard names
        pub mod errno {
            $(
                pub const $linux_const: isize = $errno;
            )*
        }

        #[repr(isize)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum $error_enum_name {
            $($error_variant = $errno,)*
            TODO = -9999,
        }

        impl ErrorTrait for $error_enum_name {
            fn from(errno: isize) -> Self {
                match errno {
                    $($errno => Self::$error_variant,)*
                    _ => Self::TODO,
                }
            }

            fn to(&self) -> (isize,isize) {
                match self {
                    $(Self::$error_variant(e) => ($errno,e),)*
                    _ => Self::TODO,
                }
            }

            fn describe(&self) -> &str {
                match *self {
                    $(Self::$error_variant => $description,)*
                    _ => "Unknown error",
                }
            }

            fn advert(&self) -> Option<isize> {
                human::info!("SYSCALL ERROR: {} - {} [{:?}]", SYSCALL_NAME, self.describe(), self);
                None
            }
        }

        impl Into<isize> for $error_enum_name {
            fn into(self) -> isize {
                self as isize
            }
        }
    };
}

pub use define_syscall_error;
pub use define_syscall_error;

/// A macro that defines an error type and error handling for syscalls.
///
/// This macro generates:
/// 1. An Error enum with the specified variants and their associated discriminant values
/// 2. ErrorTrait implementation for the Error type with proper discriminant mapping
/// 3. An discriminant module with standard Linux error constants
/// 4. Into<isize> implementation for the Error type
/// 5. A handle_result function that maps arch errors to syscall errors
///
/// # Arguments
///
/// * `$enum_identifier` - The name of the error enum (usually Error)
/// * `$variant` - The variant name in the crate::result::Error enum (e.g., Open, Read, Write)
/// * `$label` - String slice with the syscall name
/// * A list of error variants with their descriptions, discriminant values and Linux standard constant names
///   [VariantName, discriminant_value, "description", "LINUX_CONSTANT"]
///
/// # Example
///
/// ```
/// $enum_identifier:ident,
/// $variant:ty,
/// $label:expr,
/// [
///     $(
///         [
///             $discriminant:expr;
///             $identifier:ident;
///             $const_identifier:ident;
///             $acronym:expr;
///             $description:expr
///         ]
///     ),* $(,)?
/// ]
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! enum_flag {
    (
        $enum_identifier:ident,
        $variant:ty,
        $label:expr,
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $const_identifier:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $const_identifier: $variant = $discriminant;
            )*
        }

        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $enum_identifier {
            $($identifier = $discriminant,)*
            TODO,
        }

        impl $enum_identifier {
            pub fn from(discriminant: $variant) -> Self {
                match discriminant {
                    $($discriminant => Self::$identifier,)*
                    _ => Self::TODO,
                }
            }

            pub fn to(&self) -> $variant {
                match *self {
                    $(Self::$identifier => $discriminant,)*
                    _ => <$variant>::MAX
                }
            }

            pub fn str(&self) -> &str {
                match self {
                    $(Self::$identifier => $description,)*
                    _ => "TODO"
                }
            }

            pub fn acronym(&self) -> &str {
                match *self {
                    $(Self::$identifier => $acronym,)*
                    _ => "Unknown error",
                }
            }
        }



        impl core::ops::BitOr for $enum_identifier {
            type Output = usize;
            fn bitor(self, rhs: Self) -> Self::Output {
                self.to() | rhs.to()
            }
        }

        impl core::ops::Add for $enum_identifier {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() | rhs.to())
            }
        }

        impl core::ops::Sub for $enum_identifier {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() & !rhs.to())
            }
        }

        impl core::fmt::Display for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.str())
            }
        }

        impl core::fmt::Debug for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.acronym())
            }
        }
    };
}
pub use enum_flag;

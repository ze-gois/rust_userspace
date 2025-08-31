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
/// * `$enum_label` - String slice with the syscall name
/// * A list of error variants with their descriptions, discriminant values and Linux standard constant names
///   [VariantName, discriminant_value, "description", "LINUX_CONSTANT"]
///
/// # Example
///
/// ```
/// $enum_identifier:ident,
/// $variant:ty,
/// $enum_label:expr,
/// [
///     $(
///         [
///             $variant_discriminant:expr;
///             $variant_identifier:ident;
///             $variant_const_identifier:ident;
///             $variant_acronym:expr;
///             $variant_description:expr
///         ]
///     ),* $(,)?
/// ]
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! enum_labeled {
    (
        $enum_vis:vis $enum_identifier:ident,
        $enum_discriminant_type:ty,
        $enum_label:expr,
        [
            $(
                [
                    $variant_discriminant:expr;
                    $variant_identifier:ident;
                    $variant_type:ty;
                    $variant_const_identifier:ident;
                    $variant_acronym:expr;
                    $variant_description:expr
                ]
            ),* $(,)?
        ]
    ) => {

        r#enum!(
            $enum_vis $enum_identifier,
            $enum_discriminant_type, [
            $(
                [$variant_discriminant, $variant_identifier, $variant_type]
            ),*
        ]);

        pub mod constants {
            $(
                pub const $variant_const_identifier: $enum_discriminant_type = $variant_discriminant;
            )*
        }

        impl crate::traits::enums::Labeled<crate::Origin> for $enum_identifier {
            fn description(&self) -> &str {
                match self {
                    $(Self::$variant_identifier(_) => $variant_description,)*
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier(_) => $variant_acronym,)*
                }
            }
        }

        // impl core::ops::Add for $enum_identifier {
        //     type Output = Self;

        //     fn add(self, rhs: Self) -> Self::Output {
        //         $enum_identifier::from(self.discriminant() | rhs.discriminant())
        //     }
        // }

        // impl core::ops::Sub for $enum_identifier {
        //     type Output = Self;

        //     fn sub(self, rhs: Self) -> Self::Output {
        //         $enum_identifier::from(self.discriminant() & !rhs.discriminant())
        //     }
        // }

        impl core::fmt::Display for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use crate::traits::enums::Labeled;
                write!(f, "{}:{}",self.discriminant(), self.description())
            }
        }

        // impl core::fmt::Debug for $enum_identifier {
        //     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        //         write!(f, "{}:{}",self.discriminant(), self.acronym())
        //     }
        // }
    };
}
pub use enum_labeled;

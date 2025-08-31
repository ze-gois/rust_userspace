#[macro_export]
#[rustfmt::skip]
macro_rules! enum_typed {
    (
        $enum_identifier:ident;
        $variant:ty;
        $label:expr;
        $($module:tt)::*;
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $type:ty;
                    |$argumento:ident : $tipo:ty|{$($lambda:tt)*};
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
                pub const $const_identifier: $variant = $discriminant;
            )*
        }

        pub mod types {
            pub use $($module)::*::*;
            $( pub type $identifier = $type; )*
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

        #[derive(Debug, Clone, Copy)]
        pub enum EnumTyped {
            $($identifier($type),)*
            TODO((usize,usize)),
        }

        impl EnumTyped {
            pub fn from_kv(etype: *const $variant, p: *const u8) -> Self {
                match $enum_identifier::from(unsafe { *etype }) {
                    $( $enum_identifier::$identifier => EnumTyped::$identifier( unsafe { (|$argumento:$tipo|{ $($lambda)* })(p)} ),)*
                    _ => EnumTyped::TODO(  unsafe {(*etype, *(p as *const usize)) }),
                }
            }

            pub fn to_kv(&self) -> ($variant, *const u8) {
                match *self {
                    $(EnumTyped::$identifier(v) => (($enum_identifier::$identifier).to(), v as *const u8),)*
                    EnumTyped::TODO(id) => ($enum_identifier::TODO.to(), id.0 as *const u8),
                }
            }

            pub fn to_k(&self) -> $enum_identifier {
                match *self {
                    $(EnumTyped::$identifier(_) => $enum_identifier::$identifier,)*
                    EnumTyped::TODO(id) => $enum_identifier::TODO,
                }
            }

            pub fn is_null(&self) -> bool {
                match self {
                    EnumTyped::Null(0) => true,
                    _ => false,
                }
            }
        }
    };
}
pub use enum_typed;

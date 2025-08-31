#[macro_export]
#[rustfmt::skip]
macro_rules! result {
    (
        $(
            $result_identifier:ident;
            $result_label:expr;
            $result_discriminant_type:ty;
            [
                $(
                    [
                        $variant_discriminant:expr;
                        $variant_const_identifier:ident;
                        $variant_identifier:ident;
                        $($variant_type:tt)::*;
                        $variant_acronym:expr;
                        $variant_description:expr
                    ]
                ),* $(,)?
            ]
        );*
    ) => {
        $(
            r#enum!(pub $result_identifier, $result_discriminant_type,[
                $([$variant_discriminant, $variant_identifier, $($variant_type)::*],)*
            ]);
        )*

        pub mod constants {
            $( $( pub const $variant_const_identifier : $result_discriminant_type = $variant_discriminant; )* )*
        }

        pub mod acronyms {
            $( $( pub const $variant_const_identifier : &str = $variant_acronym; )* )*
        }

        pub mod descriptions {
            $( $( pub const $variant_const_identifier : &str = $variant_description; )* )*
        }

        $(
            impl crate::traits::enums::Result for $result_identifier {
                type ResultType = $result_discriminant_type;

                // fn from_no(no: Self::ResultType) -> Self {
                //     match no {
                //         $(
                //             $variant_discriminant => {
                //                 let payload : $($variant_type)::*;
                //                 payload = <$($variant_type)::*>::from_le_bytes(no.to_le_bytes());
                //                 Self::$variant_identifier(payload)
                //             },
                //         )*
                //         _ => todo!(),
                //     }
                // }

                fn acronym(&self) -> &'static str {
                    match self {
                        $(Self::$variant_identifier(_) => $variant_acronym),*
                    }
                }

                fn description(&self) -> &'static str {
                    match self {
                        $(Self::$variant_identifier(_) => $variant_description),*
                    }
                }
            }
        )*
    }
}
pub use result;

#[macro_export]
macro_rules! r#enum {
    ($enum_vis:vis $enum_identifier:ident, $enum_discriminant_type:ty, [$([$variant_discriminant:expr,$variant_identifier:ident,$($variant_type:tt)::*]),* $(,)? ]) => {
        #[derive(Debug, Clone, Copy)]
        $enum_vis enum $enum_identifier {
            $($variant_identifier($($variant_type)::*)),*
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> $enum_discriminant_type {
                match self {
                    $($enum_identifier::$variant_identifier(_) => $variant_discriminant),*
                }
            }
        }

        impl crate::traits::Bytes<crate::Origin,crate::Origin> for $enum_identifier {
            const BYTES_SIZE : usize = <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE + expressions_upperbound!($(<$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE),*);
            fn to_bytes(&self, endianness: bool) -> [u8;Self::BYTES_SIZE] {
                let mut bytes = [0u8;Self::BYTES_SIZE];

                match self {
                    $(
                        Self::$variant_identifier(payload) => {
                            let discriminant = self.discriminant();

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(&discriminant)
                                } else {
                                    <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(&discriminant)
                                }
                            );
                            o = o + <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                            bytes[o..(o+<$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(payload)
                                } else {
                                    <$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(payload)
                                }
                            );
                            bytes
                        }
                    ),*
                }
            }

            fn from_bytes(bytes: [u8;Self::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                discriminant_bytes.copy_from_slice(&bytes[o..(o+<$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)]);
                let discriminant = if endianness {
                    <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(discriminant_bytes)
                } else {
                    <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(discriminant_bytes)
                };
                o = o + <$enum_discriminant_type as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier({
                                let mut payload = [0u8; <$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                                payload.copy_from_slice(&bytes[o..(o+<$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)]);
                                if endianness {
                                    <$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(payload)
                                } else {
                                    <$($variant_type)::* as crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(payload)
                                }
                            })
                        },
                    )*
                    _ => unreachable!()
                }

            }
        }
    };
}
pub use r#enum;

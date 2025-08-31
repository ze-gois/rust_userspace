#[macro_export]
macro_rules! r#struct {
    ($vis:vis $struct_identifier:ident {$($field_visibility:vis $field_identifier:ident: $field_type:ty),*$(,)?}) => {

        // #[repr(C,packed)]
        #[derive(Debug, Clone, Copy)]
        $vis struct $struct_identifier {
            $($field_visibility $field_identifier: $field_type),*
        }

        impl $crate::traits::Bytes<crate::Origin,crate::Origin> for $struct_identifier {
            const BYTES_SIZE : usize = $(<$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE +)* 0;

            fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
                let mut b = [0u8; Self::BYTES_SIZE];
                let mut o = 0;
                $(
                    b[o..(o+<$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)].copy_from_slice(
                        &if endianness {
                            <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(&self.$field_identifier)
                        } else {
                            <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(&self.$field_identifier)
                        }
                    );
                    o = o + <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                )*
                b
            }

            fn from_bytes(bytes : [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                $(
                    let mut field_bytes = [0u8; <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    field_bytes.copy_from_slice(&bytes[o..(o+<$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE)]);
                    let $field_identifier = if endianness {
                        <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(field_bytes)
                    } else {
                        <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(field_bytes)
                    };
                    o = o + <$field_type as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                )*
                Self {
                    $($field_identifier,)*
                }
            }
        }

        type OptionDiscriminant = u8;
        impl $crate::traits::Bytes<crate::Origin,crate::Origin> for Option<$struct_identifier> {
            const BYTES_SIZE: usize = <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE + <$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
            fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
                let mut option_bytes = [0u8; <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                let mut o = 0;
                let mut l = <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                option_bytes.copy_from_slice(&bytes[o..l]);
                let option = if endianness {
                    <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(option_bytes)
                } else {
                    <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(option_bytes)
                };
                if option == 0 {
                    None
                } else {
                    o = l;
                    l = l + <$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                    let mut value_bytes = [0u8; <$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    value_bytes.copy_from_slice(&bytes[o..l]);
                    if endianness {
                        Some(<$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(value_bytes))
                    } else {
                        Some(<$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(value_bytes))
                    }
                }
            }

            fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
                let mut bytes = [0u8; Self::BYTES_SIZE];
                if let Some(v) = self {
                    let mut o = 0;
                    let mut l = <OptionDiscriminant as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                    bytes[o..l].copy_from_slice(&(1 as OptionDiscriminant).to_le_bytes());
                    o = l;
                    l = l + <$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                    if endianness {
                        bytes[o..l].copy_from_slice(&<$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(v));
                    } else {
                        bytes[o..l].copy_from_slice(&<$struct_identifier as $crate::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(v));
                    }
                    bytes
                } else {
                    bytes
                }
            }
        }
    }
}
pub use r#struct;

pub type Unit = ();

// impl crate::traits::Primitive for Unit {
//     const IS_PRIMITIVE: bool = true;
// }

// impl crate::traits::Bytes for Unit {
//     const BYTES_SIZE: usize = 0;

//     fn to_bytes(&self, _endianness: bool) -> [u8; Self::BYTES_SIZE] {
//         [0u8; 0]
//     }

//     fn from_bytes(_bytes: [u8; Self::BYTES_SIZE], _endianness: bool) -> Self {
//         ()
//     }
// }

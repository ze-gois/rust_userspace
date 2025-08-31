pub trait Bytes<Observer, Reference> {
    const BYTES_SIZE: usize;

    fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE];
    fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE] {
        self.to_bytes(true)
    }
    fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE] {
        self.to_bytes(false)
    }

    fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self;
    fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
    where
        Self: Sized,
    {
        Self::from_bytes(bytes, true)
    }
    fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
    where
        Self: Sized,
    {
        Self::from_bytes(bytes, false)
    }
}

// pub trait Bytes<Observer, Reference> {
//     const BYTES_SIZE: usize;

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:;
//     fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:,
//     {
//         self.to_bytes(true)
//     }
//     fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:,
//     {
//         self.to_bytes(false)
//     }
//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//     where
//         [u8; Self::BYTES_SIZE]:;
//     fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//         [u8; Self::BYTES_SIZE]:,
//     {
//         Self::from_bytes(bytes, true)
//     }
//     fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//         [u8; Self::BYTES_SIZE]:,
//     {
//         Self::from_bytes(bytes, false)
//     }
// }

// pub trait Bytes<Observer, Reference> {
//     const BYTES_SIZE: usize;

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE];
//     fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//         self.to_bytes(true)
//     }
//     fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//         self.to_bytes(false)
//     }
//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self;
//     fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//     {
//         Self::from_bytes(bytes, true)
//     }
//     fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//     {
//         Self::from_bytes(bytes, false)
//     }
// }

//! Decoding of serialized ELF scalar representations.
//!
//! ELF byte order is selected by `e_ident[EI_DATA]`; decoding therefore
//! belongs to the serialized representation layer rather than to the host
//! architecture.

use super::super::identification::Data;

#[derive(Debug, Clone, Copy)]
pub struct Decoder<'bytes> {
    bytes: &'bytes [u8],
    offset: usize,
    data: Data,
}

impl<'bytes> Decoder<'bytes> {
    pub fn new(bytes: &'bytes [u8], offset: usize, data: Data) -> Option<Self> {
        match data {
            Data::LeastSignificantByteFirst | Data::MostSignificantByteFirst => {
                if offset <= bytes.len() {
                    Some(Self {
                        bytes,
                        offset,
                        data,
                    })
                } else {
                    None
                }
            }
            Data::None | Data::Reserved(_) => None,
        }
    }

    pub const fn position(&self) -> usize {
        self.offset
    }

    pub fn byte(&mut self) -> Option<u8> {
        let byte = *self.bytes.get(self.offset)?;
        self.offset = self.offset.checked_add(1)?;
        Some(byte)
    }

    pub fn bytes<const N: usize>(&mut self) -> Option<[u8; N]> {
        let end = self.offset.checked_add(N)?;
        let bytes: [u8; N] = self.bytes.get(self.offset..end)?.try_into().ok()?;
        self.offset = end;
        Some(bytes)
    }

    pub fn half(&mut self) -> Option<u16> {
        let bytes = self.bytes::<2>()?;
        Some(match self.data {
            Data::LeastSignificantByteFirst => u16::from_le_bytes(bytes),
            Data::MostSignificantByteFirst => u16::from_be_bytes(bytes),
            Data::None | Data::Reserved(_) => return None,
        })
    }

    pub fn word(&mut self) -> Option<u32> {
        let bytes = self.bytes::<4>()?;
        Some(match self.data {
            Data::LeastSignificantByteFirst => u32::from_le_bytes(bytes),
            Data::MostSignificantByteFirst => u32::from_be_bytes(bytes),
            Data::None | Data::Reserved(_) => return None,
        })
    }

    pub fn sword(&mut self) -> Option<i32> {
        let bytes = self.bytes::<4>()?;
        Some(match self.data {
            Data::LeastSignificantByteFirst => i32::from_le_bytes(bytes),
            Data::MostSignificantByteFirst => i32::from_be_bytes(bytes),
            Data::None | Data::Reserved(_) => return None,
        })
    }

    pub fn xword(&mut self) -> Option<u64> {
        let bytes = self.bytes::<8>()?;
        Some(match self.data {
            Data::LeastSignificantByteFirst => u64::from_le_bytes(bytes),
            Data::MostSignificantByteFirst => u64::from_be_bytes(bytes),
            Data::None | Data::Reserved(_) => return None,
        })
    }

    pub fn sxword(&mut self) -> Option<i64> {
        let bytes = self.bytes::<8>()?;
        Some(match self.data {
            Data::LeastSignificantByteFirst => i64::from_le_bytes(bytes),
            Data::MostSignificantByteFirst => i64::from_be_bytes(bytes),
            Data::None | Data::Reserved(_) => return None,
        })
    }
}

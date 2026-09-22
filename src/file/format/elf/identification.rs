//! ELF identification bytes (`e_ident`).

pub const SIZE: usize = 16;

pub const MAGIC_0_INDEX: usize = 0;
pub const MAGIC_1_INDEX: usize = 1;
pub const MAGIC_2_INDEX: usize = 2;
pub const MAGIC_3_INDEX: usize = 3;
pub const CLASS_INDEX: usize = 4;
pub const DATA_INDEX: usize = 5;
pub const VERSION_INDEX: usize = 6;
pub const OS_ABI_INDEX: usize = 7;
pub const ABI_VERSION_INDEX: usize = 8;
pub const PADDING_INDEX: usize = 9;

pub const MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    None = 0,
    Class32 = 1,
    Class64 = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Data {
    None = 0,
    LeastSignificantByteFirst = 1,
    MostSignificantByteFirst = 2,
}

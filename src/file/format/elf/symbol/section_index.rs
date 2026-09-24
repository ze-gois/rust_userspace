//! Resolved section index associated with an ELF symbol.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSectionIndex {
    Undefined,
    Section(usize),
    ProcessorSpecific(u16),
    OperatingSystemSpecific(u16),
    Absolute,
    Common,
    Reserved(u16),
}

impl ResolvedSectionIndex {
    pub fn resolve(raw: super::super::section_header::Index, extended: Option<u32>) -> Option<Self> {
        match raw.raw() {
            0 => Some(Self::Undefined),
            0x0001..=0xfeff => Some(Self::Section(raw.raw() as usize)),
            0xff00..=0xff1f => Some(Self::ProcessorSpecific(raw.raw())),
            0xff20..=0xff3f => Some(Self::OperatingSystemSpecific(raw.raw())),
            0xfff1 => Some(Self::Absolute),
            0xfff2 => Some(Self::Common),
            0xffff => Some(Self::Section(usize::try_from(extended?).ok()?)),
            value => Some(Self::Reserved(value)),
        }
    }
}

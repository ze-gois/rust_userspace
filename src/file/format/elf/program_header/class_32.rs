use super::super::representation::class_32 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub p_type: representation::Word,
    pub p_offset: representation::Offset,
    pub p_vaddr: representation::Address,
    pub p_paddr: representation::Address,
    pub p_filesz: representation::Word,
    pub p_memsz: representation::Word,
    pub p_flags: representation::Word,
    pub p_align: representation::Word,
}

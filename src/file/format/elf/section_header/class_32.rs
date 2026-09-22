use super::super::representation::class_32 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub sh_name: representation::Word,
    pub sh_type: representation::Word,
    pub sh_flags: representation::Word,
    pub sh_addr: representation::Address,
    pub sh_offset: representation::Offset,
    pub sh_size: representation::Word,
    pub sh_link: representation::Word,
    pub sh_info: representation::Word,
    pub sh_addralign: representation::Word,
    pub sh_entsize: representation::Word,
}

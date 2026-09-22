use super::super::representation::class_64 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub sh_name: representation::Word,
    pub sh_type: representation::Word,
    pub sh_flags: representation::Xword,
    pub sh_addr: representation::Address,
    pub sh_offset: representation::Offset,
    pub sh_size: representation::Xword,
    pub sh_link: representation::Word,
    pub sh_info: representation::Word,
    pub sh_addralign: representation::Xword,
    pub sh_entsize: representation::Xword,
}

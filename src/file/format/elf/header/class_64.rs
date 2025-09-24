use crate::file::format::elf::dtype::class_64::*;

ample::r#struct!(
    pub struct Header64 {
        pub e_ident: super::Identifier,
        pub e_type: Half,
        pub e_machine: Half,
        pub e_version: Word,
        pub e_entry: Addr,
        pub e_phoff: Off,
        pub e_shoff: Off,
        pub e_flags: Word,
        pub e_ehsize: Half,
        pub e_phentsize: Half,
        pub e_phnum: Half,
        pub e_shentsize: Half,
        pub e_shnum: Half,
        pub e_shstrndx: Half,
    }
);

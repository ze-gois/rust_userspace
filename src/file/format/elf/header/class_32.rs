use crate::file::format::elf::dtype::class_32::*;

ample::r#struct!(
    pub struct Header32 {
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

impl core::fmt::Debug for Header32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Header32 {{\n")?;
        write!(f, "\tident     ({:?})\n", self.e_ident)?;
        write!(f, "\ttype      ({:?})\n", self.e_type)?;
        write!(f, "\tmachine   ({:?})\n", self.e_machine)?;
        write!(f, "\tversion   ({:?})\n", self.e_version)?;
        write!(f, "\tentry     ({:?})\n", self.e_entry)?;
        write!(f, "\tphoff     ({:?})\n", self.e_phoff)?;
        write!(f, "\tshoff     ({:?})\n", self.e_shoff)?;
        write!(f, "\tflags     ({:?})\n", self.e_flags)?;
        write!(f, "\tehsize    ({:?})\n", self.e_ehsize)?;
        write!(f, "\tphentsize ({:?})\n", self.e_phentsize)?;
        write!(f, "\tphnum     ({:?})\n", self.e_phnum)?;
        write!(f, "\tshentsize ({:?})\n", self.e_shentsize)?;
        write!(f, "\tshnum     ({:?})\n", self.e_shnum)?;
        write!(f, "\tshstrndx  ({:?})\n", self.e_shstrndx)?;
        write!(f, "}} Header64")
    }
}

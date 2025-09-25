extern crate alloc;

pub enum SpecialName {
    Fixed(&'static str),
    RelPrefix,  // apenas o prefixo ".rel"
    RelaPrefix, // apenas o prefixo ".rela"
}

impl SpecialName {
    /// Gera o nome completo combinando com o alvo fornecido
    pub fn generate(&self, target: &str) -> alloc::string::String {
        match self {
            SpecialName::Fixed(s) => (*s).into(),
            SpecialName::RelPrefix => [".rel", target].concat().into(),
            SpecialName::RelaPrefix => [".rela", target].concat().into(),
        }
    }
}

pub struct Special {
    pub name: SpecialName,
    pub types: &'static [super::Type],
    pub flags: &'static [super::Flag],
}

#[rustfmt::skip]
pub const SPECIALS: &'static [Special] = &[
    Special { name: SpecialName::Fixed(".bss"),           types : &[super::Type::Nobits],       flags : &[super::Flag::Alloc, super::Flag::Write]},
    Special { name: SpecialName::Fixed(".comment"),       types : &[super::Type::Progbits],     flags : &[] },
    Special { name: SpecialName::Fixed(".data"),          types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Write] },
    Special { name: SpecialName::Fixed(".data1"),         types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Write] },
    Special { name: SpecialName::Fixed(".debug"),         types : &[super::Type::Progbits],     flags : &[] },
    Special { name: SpecialName::Fixed(".dynamic"),       types : &[super::Type::Dynamic],      flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".dynstr"),        types : &[super::Type::Strtab],       flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".dynsym"),        types : &[super::Type::Dynsym],       flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".fini"),          types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Execinstr] },
    Special { name: SpecialName::Fixed(".fini_array"),    types : &[super::Type::FiniArray],    flags : &[super::Flag::Alloc, super::Flag::Write] },
    Special { name: SpecialName::Fixed(".got"),           types : &[super::Type::Progbits],     flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".hash"),          types : &[super::Type::Hash],         flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".init"),          types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Execinstr] },
    Special { name: SpecialName::Fixed(".init_array"),    types : &[super::Type::InitArray],    flags : &[super::Flag::Alloc, super::Flag::Write] },
    Special { name: SpecialName::Fixed(".interp"),        types : &[super::Type::Progbits],     flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".line"),          types : &[super::Type::Progbits],     flags : &[] },
    Special { name: SpecialName::Fixed(".note"),          types : &[super::Type::Note],         flags : &[] },
    Special { name: SpecialName::Fixed(".plt"),           types : &[super::Type::Progbits],     flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".preinit_array"), types : &[super::Type::PreinitArray], flags : &[super::Flag::Alloc, super::Flag::Write] },
    Special { name: SpecialName::RelPrefix,               types : &[super::Type::Rel],          flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::RelaPrefix,              types : &[super::Type::Rela],         flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".relr.dyn"),      types : &[super::Type::Relr],         flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".rodata"),        types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".rodata1"),       types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc] },
    Special { name: SpecialName::Fixed(".shstrtab"),      types : &[super::Type::Strtab],       flags : &[] },
    Special { name: SpecialName::Fixed(".strtab"),        types : &[super::Type::Strtab],       flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".symtab"),        types : &[super::Type::Symtab],       flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".symtab_shndx"),  types : &[super::Type::SymtabShndx],  flags : &[] }, //super::Flag::Below
    Special { name: SpecialName::Fixed(".tbss"),          types : &[super::Type::Nobits],       flags : &[super::Flag::Alloc, super::Flag::Write, super::Flag::Tls] },
    Special { name: SpecialName::Fixed(".tdata"),         types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Write, super::Flag::Tls] },
    Special { name: SpecialName::Fixed(".tdata1"),        types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Write, super::Flag::Tls] },
    Special { name: SpecialName::Fixed(".text"),          types : &[super::Type::Progbits],     flags : &[super::Flag::Alloc, super::Flag::Execinstr] },
];

// pub fn get_special(header: &super::Header64, shstrtab: &[u8]) -> Option<&'static Special> {
//     let sh_name = header_name(header, shstrtab);

//     let Some(sh_type) = header.r#type() else {
//         return None;
//     };

//     let Some(sh_flags) = header.flags() else {
//         return None;
//     };

//     for special in SPECIALS {
//         if !match &special.name {
//             SpecialName::Fixed(s) => s == &sh_name,
//             SpecialName::RelPrefix => sh_name.starts_with(".rel"),
//             SpecialName::RelaPrefix => sh_name.starts_with(".rela"),
//         } {
//             continue;
//         }

//         for r#type in special.types.iter() {
//             if !(sh_type && r#type) {
//                 continue;
//             };
//         }

//         for flag in special.flags.iter() {
//             if !(sh_flags && *flag) {
//                 continue;
//             };
//         }

//         return Some(special);
//     }
//     None
// }

// .sdata
// .tdesc
// .sbss
// .lit4
// .lit8
// .reginfo
// .gptab
// .liblist
// .conflict

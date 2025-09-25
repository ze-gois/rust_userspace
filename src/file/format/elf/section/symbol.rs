pub mod class_32;
pub mod class_64;

pub use class_32::Symbol32;
pub use class_64::Symbol64;

// #define ELF32_ST_BIND(i) ((i)>>4)
// #define ELF32_ST_TYPE(i) ((i)&0xf)
// #define ELF32_ST_INFO(b,t) (((b)<<4)+((t)&0xf))
// #define ELF64_ST_BIND(i) ((i)>>4)
// #define ELF64_ST_TYPE(i) ((i)&0xf)
// #define ELF64_ST_INFO(b,t) (((b)<<4)+((t)&0xf))

// #define ELF32_ST_VISIBILITY(o) ((o)&0x7)
// #define ELF64_ST_VISIBILITY(o) ((o)&0x7)

// STB_LOCAL 0
// STB_GLOBAL 1
// STB_WEAK 2
// STB_LOOS 10
// STB_HIOS 12
// STB_LOPROC 13
// STB_HIPROC 15
//
//
// STT_NOTYPE 0
// STT_OBJECT 1
// STT_FUNC 2
// STT_SECTION 3
// STT_FILE 4
// STT_COMMON 5
// STT_TLS 6
// STT_LOOS 10
// STT_HIOS 12
// STT_LOPROC 13
// STT_HIPROC 15
//
//
// STV_DEFAULT 0
// STV_INTERNAL 1
// STV_HIDDEN 2
// STV_PROTECTED 3
// STV_EXPORTED 4
// STV_SINGLETON 5
// STV_ELIMINATE 6
//

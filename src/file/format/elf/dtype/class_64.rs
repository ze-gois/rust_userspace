// use super::ELFType;

crate::file_format_elf_dtype_define!(pub Null, u8); //Unsigned file offset
crate::file_format_elf_dtype_define!(pub UChar, u8); //Unsigned file offset
crate::file_format_elf_dtype_define!(pub SXWord, i64); //Unsigned program address
crate::file_format_elf_dtype_define!(pub Half, u16); //Unsigned medium integer
crate::file_format_elf_dtype_define!(pub SWord, i32); //Unsigned integer
crate::file_format_elf_dtype_define!(pub XWord, u64); //Signed integer
crate::file_format_elf_dtype_define!(pub Word, u32); //Unsigned long integer
crate::file_format_elf_dtype_define!(pub Off, u64); //Signed long integer
crate::file_format_elf_dtype_define!(pub Addr, u64); //Unsigned small integer

// file_format_elf_dtype_impl!(Null, UChar, SXWord, Half, SWord, XWord, Word, Off, Addr);

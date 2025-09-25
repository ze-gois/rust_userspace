// DT_NULL 0 ignored mandatory mandatory
// DT_NEEDED 1 d_val optional optional
// DT_PLTRELSZ 2 d_val optional optional
// DT_PLTGOT 3 d_ptr optional optional
// DT_HASH 4 d_ptr mandatory† mandatory†
// DT_STRTAB 5 d_ptr mandatory mandatory
// DT_SYMTAB 6 d_ptr mandatory mandatory
// DT_RELA 7 d_ptr mandatory optional
// DT_RELASZ 8 d_val mandatory optional
// DT_RELAENT 9 d_val mandatory optional
// DT_STRSZ 10 d_val mandatory mandatory
// DT_SYMENT 11 d_val mandatory mandatory
// DT_INIT 12 d_ptr optional optional
// DT_FINI 13 d_ptr optional optional
// DT_SONAME 14 d_val ignored optional
// DT_RPATH* 15 d_val optional ignored
// DT_SYMBOLIC* 16 ignored ignored optional
// DT_REL 17 d_ptr mandatory optional
// DT_RELSZ 18 d_val mandatory optional
// DT_RELENT 19 d_val mandatory optional
// DT_PLTREL 20 d_val optional optional
// DT_DEBUG 21 d_ptr optional ignored
// DT_TEXTREL* 22 ignored optional optional
// DT_JMPREL 23 d_ptr optional optional
// DT_BIND_NOW* 24 ignored optional optional
// DT_INIT_ARRAY 25 d_ptr optional optional
// DT_FINI_ARRAY 26 d_ptr optional optional
// DT_INIT_ARRAYSZ 27 d_val optional optional
// DT_FINI_ARRAYSZ 28 d_val optional optional
// DT_RUNPATH 29 d_val optional optional
// DT_FLAGS 30 d_val optional optional
// DT_ENCODING 32 unspecified unspecified unspecified
// DT_PREINIT_ARRAY 32 d_ptr optional ignored
// DT_PREINIT_ARRAYSZ 33 d_val optional ignored
// DT_SYMTAB_SHNDX 34 d_ptr optional optional
// DT_RELRSZ 35 d_val optional optional
// DT_RELR 36 d_ptr optional optional
// DT_RELRENT 37 d_val optional optional
// DT_SYMTABSZ 39 d_val optional† optional†
// DT_LOOS 0x6000000D unspecified unspecified unspecified
// DT_HIOS 0x6ffff000 unspecified unspecified unspecified
// DT_LOPROC 0x70000000 unspecified unspecified unspecified
// DT_HIPROC 0x7fffffff unspecified unspecified unspecified

// DF_ORIGIN 0x1
// DF_SYMBOLIC 0x2
// DF_TEXTREL 0x4
// DF_BIND_NOW 0x8
// DF_STATIC_TLS 0x10

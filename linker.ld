ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .text : ALIGN(0x1000) {
        PROVIDE(_text_start = .);
        *(.text._start)
        *(.text*)
        PROVIDE(_text_end = .);
    } :text

    .rodata : ALIGN(0x1000) {
        PROVIDE(_rodata_start = .);
        *(.rodata*)
        PROVIDE(_rodata_end = .);
    } :rodata

    .got : ALIGN(8) {
        PROVIDE(_got_start = .);
        *(.got)
        *(.got.*)
        PROVIDE(_got_end = .);
    } :data

    .data : ALIGN(0x1000) {
        PROVIDE(_data_start = .);
        *(.data*)
        *(.data.*)
        *(.sdata*)
        PROVIDE(_data_end = .);
    } :data

    .dynamic : {
        PROVIDE(_dynamic_start = .);
        *(.dynamic)
        PROVIDE(_dynamic_end = .);
    } :data

    .bss : ALIGN(0x1000) {
        PROVIDE(_bss_start = .);
        *(.bss*)
        *(.bss.*)
        *(.sbss*)
        *(COMMON)
        . = ALIGN(16);
        PROVIDE(_bss_end = .);
    } :data

    /DISCARD/ : {
        *(.comment)
        *(.debug_*)
        *(.note.*)
        *(.gcc_except_table)
        *(.symtab)
        *(.strtab)
    }
}

PROVIDE(__bss_size = _bss_end - _bss_start);

PHDRS {
    text    PT_LOAD FLAGS(5);
    rodata  PT_LOAD FLAGS(4);
    data    PT_LOAD FLAGS(6);
}

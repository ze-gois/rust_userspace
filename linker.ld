ENTRY(_start)

/*
 * Static PIE userspace image.
 *
 * Link-time virtual addresses describe the image; the kernel or our ELF loader
 * may choose a different load address. All runtime references must therefore
 * remain position independent and the load bias is expressed through the ELF
 * base address.
 */
SECTIONS {
    . = 0x1000;

    .text : ALIGN(0x1000) {
        PROVIDE(_text_start = .);
        *(.text._start)
        *(.text*)
        PROVIDE(_text_end = .);
    } :text

    .rodata : ALIGN(0x1000) {
        PROVIDE(_rodata_start = .);
        *(.rodata*)
        *(.rodata.*)
        *(.eh_frame)
        *(.eh_frame.*)
        *(.dynsym)
        *(.dynstr)
        *(.hash)
        *(.gnu.hash)
        *(.rela.dyn)
        *(.rela.plt)
        PROVIDE(_rodata_end = .);
    } :rodata

    .got : ALIGN(8) {
        PROVIDE(_got_start = .);
        *(.got)
        *(.got.*)
        *(.got.plt)
        PROVIDE(_got_end = .);
    } :data

    .data : ALIGN(0x1000) {
        PROVIDE(_data_start = .);
        *(.data*)
        *(.data.*)
        *(.sdata*)
        PROVIDE(_data_end = .);
    } :data

    .dynamic : ALIGN(8) {
        PROVIDE(_dynamic_start = .);
        *(.dynamic)
        PROVIDE(_dynamic_end = .);
    } :data :dynamic

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
    }
}

PROVIDE(__bss_size = _bss_end - _bss_start);

PHDRS
{
    text    PT_LOAD    FLAGS(5);
    rodata  PT_LOAD    FLAGS(4);
    data    PT_LOAD    FLAGS(6);
    dynamic PT_DYNAMIC FLAGS(6);
}

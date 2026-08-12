.bss
.balign 4096
.globl fixture_bss
.type fixture_bss,@object
fixture_bss:
    .zero 131072

.text
.globl _start
.type _start,@function
_start:
    mov $60, %rax
    mov $42, %rdi
    syscall
    hlt

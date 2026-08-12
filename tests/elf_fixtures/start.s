.text
.globl _start
.type _start,@function
_start:
    mov $60, %rax
    mov $42, %rdi
    syscall
    hlt

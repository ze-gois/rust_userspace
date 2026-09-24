   .text
   .align  4
   .globl  _start
   .hidden _start
   .section .text._start
   .type   _start,@function
   _start:
       # Preserve the initial process stack supplied by the kernel or by our
       # own ELF loader. r12 is callee-saved by the System V x86-64 ABI.
       mov     %rsp, %r12

       # The stack must be 16-byte aligned immediately before a call.
       and     $-16, %rsp
       xor     %ebp, %ebp

       # Static PIE: resolve linker-provided BSS boundaries relative to RIP
       # rather than embedding absolute virtual addresses.
       lea     _bss_start(%rip), %rax
       lea     _bss_end(%rip), %rcx
       cmp     %rcx, %rax
       je      bss_init_done

bss_zero_loop:
       movq    $0, (%rax)
       add     $8, %rax
       cmp     %rcx, %rax
       jb      bss_zero_loop

bss_init_done:
       # Pass the untouched Linux initial stack to the application entry point.
       mov     %r12, %rdi
       call    entry

       # entry has return type `!`; reaching here indicates a contract violation.
       ud2

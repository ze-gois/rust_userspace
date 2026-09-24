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

       # Static PIE: resolve linker-provided BSS boundaries relative to RIP.
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
       # The Linux kernel maps a static ET_DYN image but does not perform its
       # dynamic relocations. Before entering Rust, apply the same x86-64
       # R_X86_64_RELATIVE semantics used by our ELF process-image loader.
       #
       # Elf64_Rela:
       #   +0   r_offset
       #   +8   r_info
       #   +16  r_addend
       #   size 24
       #
       # R_X86_64_RELATIVE has type 8, symbol index 0, value B + A.
       lea     _image_start(%rip), %rbx
       lea     _rela_dyn_start(%rip), %rsi
       lea     _rela_dyn_end(%rip), %rdi

self_relocation_loop:
       cmp     %rdi, %rsi
       jae     self_relocation_done

       mov     8(%rsi), %rax
       mov     %eax, %ecx
       test    %ecx, %ecx
       je      self_relocation_next
       cmp     $8, %ecx
       jne     self_relocation_unsupported
       shr     $32, %rax
       test    %rax, %rax
       jne     self_relocation_unsupported

       mov     0(%rsi), %rdx
       add     %rbx, %rdx
       mov     16(%rsi), %rax
       add     %rbx, %rax
       mov     %rax, (%rdx)

self_relocation_next:
       add     $24, %rsi
       jmp     self_relocation_loop

self_relocation_unsupported:
       ud2

self_relocation_done:
       # Pass the untouched Linux initial stack to the application entry point.
       mov     %r12, %rdi
       call    entry

       # entry has return type `!`; reaching here indicates a contract violation.
       ud2

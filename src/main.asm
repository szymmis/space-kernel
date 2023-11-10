[BITS 32] ; kernel code is running in protected (32bit) mode
[EXTERN main] ; main comes from C code
[EXTERN kb_intr_handler]
[EXTERN timer_intr_handler]

call init_idt ; Call the IDT initialization routine from idt.asm

call main ; Invoke kernel C main() entry point
jmp $ ; Loop forever in case of kernel return

%include "idt.asm"

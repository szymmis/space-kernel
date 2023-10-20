[BITS 32] ; kernel code is running in protected (32bit) mode
[EXTERN main] ; main comes from C code

call main ; Invoke kernel C main() entry point
jmp $ ; Loop forever in case of kernel return

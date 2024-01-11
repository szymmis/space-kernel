[ORG 0x7C00] ; BIOS loads the boot sector into 0x7C00
[BITS 16] ; Real mode is 16-bit

%include "disk/read_disk.asm"
%include "pm/switch_to_pm.asm"
%include "pm/gdt.asm"

[BITS 32] ; Kernel is running in 32-bit protected mode
start_kernel:	
	call 0x8000 ; Jump to code loaded in read_disk.asm
	jmp $ ; Loop forever to prevent execution of random data

TIMES 510 - ($ - $$) db 0 ; Fill the rest of sector with 0s up to 510 bytes
DW 0xAA55 ; Last two bytes form the BIOS boot magic number

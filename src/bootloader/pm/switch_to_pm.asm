; This code is responsible for setting up VGA video mode using BIOS interrupts
; when still in real mode and then switching to protected mode 
; by loading the GDT defined in gdt.asm and setting the protected mode flag in cr0 register
;
; See https://wiki.osdev.org/Protected_mode for more information
set_graphics_mode:
	mov ah, 0x00 ; Function code of 'Set video mode' BIOS interrupt
				 ; For more information see https://en.wikipedia.org/wiki/INT_10H
	mov al, 0x03 ; Parameter defining what mode to set - 0x03 = Text mode
	; mov al, 0x13 ; 0x13 - VGA 320x200 256 colors mode
	int 0x10 ; BIOS interrupt

switch_to_pm:
	; ** Load GDT **
	lgdt [gdtr_descriptor] 
	; ** Disable interrupts **
	cli 

	; ** Enable protected mode flag**
	mov eax, cr0 ; cr0 is the register holding CPU flags
	or eax, 1 ; First bit of the register is the protected mode flag
	mov cr0, eax

	; ** Reload data segment registers **
	; Every segment register is loaded with the data segment offset
	mov ax, 0x10 ; 0x10 (16 bytes) - data segment descriptor offset in the GDT
	mov ds, ax 
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	; ** Jump to the protected mode **
	; The jump needs to be done to reload the CS register with proper PM32 descriptior
	jmp 0x08:start_kernel ; 0x08 (8 bytes) - code segment descriptor offset in the GDT

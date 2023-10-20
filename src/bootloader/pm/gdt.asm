; Each entry in the GDT is 8 bytes long (64 bits)
; The fist entry of the GDT needs to be null

; Both code and data segments are of maximum size (0xFFFFF)
; and are overlapping (both start at offset 0x0)
;
; For more info see: https://wiki.osdev.org/Global_Descriptor_Table#Segment_Descriptor
gdt_start:
	gdt_null:
		dq 0x0 ; Write 64 bits of 0s
	
	; Code segment descriptor
	gdt_code:
		dw 0xFFFF ; 16 bits value defining segment limit (size)
		dw 0x0 ; First 24 bits
		db 0x0 ; of segment offset
		; Access byte defining segment type
		db 10011010b ; ....1...b = Segment is executable (code)
		db 11001111b ; First (lower) 4 bits are offset, last 4 bits are flags
					 ; describing that segment is 32 bits and limit is in 4KiB blocks
					 ; instead of 1 Byte blocks 
		db 0x0 ; Last 8 bits of segment offset

	; Data segment descriptor
	gdt_data:
		dw 0xFFFF
		dw 0x0
		db 0x0
		db 10010010b ; ....0...b = Segment is writable (data)
		db 11001111b
		db 0x0

gdt_end:

; This label is used by the lgdt instruction in switch_to_pm.asm
gdtr_descriptor: 
	dw gdt_end - gdt_start - 1 ; Size of the GDT descriptor
	dd gdt_start ; Address of the GDT descriptor

; Code is loaded at 0x8000 (segment 0x0000, offset 0x8000) from floppy
; using BIOS interrupt 0x02 (read sectors from drive)
load_kernel_code:
    ; Read kernel from disk
    mov ah, 0x02 ; AH=2 - read sectors from drive
    mov al, 64 ; Number of sectors to read
    mov ch, 0 ; Cylinder number
    mov cl, 2 ; Sector number
    mov dh, 0 ; Head number
    mov dl, 0 ; Drive number (0=A, 1=B)

    ; ES:BX -> Buffer to read sectors to
    mov bx, 0x0000 ; 0x0000 is the offset of the code segment offset (not descriptor offset 0x08) defined in gdt.asm
    mov es, bx 
    mov bx, 0x8000 ; 0x8000 is an arbitrary address where the kernel will be loaded
                   ; It's value is picked based on that boodloader is loaded at 0x7C00 by BIOS
                   ; and its 512 bytes long so the last address is 0x7E00 so we peek 0x8000 as the next
                   ; "round" number. Worth noting is that the next reserved address is 0xA0000 where
                   ; VGA buffer is located so we have 0xA0000 - 0x8000 = 622592 bytes (608 KiB) of space

    int 0x13 ; Invoke BIOS interrupt

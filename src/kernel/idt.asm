; Macro for defining an IDT entry
; %1 is the interrupt code
; %2 is the handler label
; Handler address (32bits) that is loaded into eax needs to be split into two 16bit parts
;
; See https://wiki.osdev.org/Interrupt_Descriptor_Table#Structure_on_IA-32 for IDT entry structure
%macro idt_entry 2
    mov eax, %2
    mov [idt_start + %1 * 8],ax
    mov word [idt_start + %1 * 8 + 2],0x0008 ; 0x0008 is the code segment descriptor offset
    mov word [idt_start + %1 * 8 + 4],0x8E00 ; Lower 16 bits (0x__00) are reserved, upper (0x8E__) are flags
    shr eax, 16 ; Shift eax right by 16 bits to extract the upper 16 bits of the handler address
    mov [idt_start + %1 * 8 + 6],ax
%endmacro

init_idt:
    lidt [idtr_descriptor] ; Load the IDT register with descriptor information

    ; PIC needs to be initialized before hardware (timer and keyboard) interrupts can be used
    ;
    ; See https://wiki.osdev.org/PIC#Spurious_IRQs 
    ;     https://wiki.osdev.org/Interrupts#Standard_ISA_IRQs
    mov al, 0x11 ; Send init command (0x11)
    out 0x20, al ; to PIC master command port (0x20)
                 ; After that, the PIC expects 3 more bytes
    
    ; First byte is the offset of the interrupt vector
    ; In this case, the IRQ0 (timer) will be sent as interrupt 0x20 instead of 0x00
    ; It needs to be done due to overlap of PIC and CPU interrupts
    ;
    ; See https://wiki.osdev.org/PIC#Protected_Mode for more details
    mov al, 0x20
    out 0x21, al
    ; Second byte is used for interrupts masking
    ; This doesn't seem to be working right now
    ; See https://wiki.osdev.org/PIC#Masking for more details
    mov eax, 0xFFFFFFFF
    out 0x21, eax

    idt_entry 0x20, timer_int ; Handler for IRQ 0 (timer)
    idt_entry 0x21, keyboard_int ; Handler for IRQ 1 (keyboard)

    sti ; Enable interrupts that were disabled during protected mode switch

    ret ; Return to the caller (kernel_entry.asm)

idt_start:
    resd 50*2 ; Reserve 50*2 dwords (each entry is 8 bytes) for IDT entries
 
idtr_descriptor:
    dw (50*8)-1 ; Size of the IDT is 50 entries - 1 (IDT is 0-indexed)
    dd idt_start ; Address of the IDT

keyboard_int:
    call kb_intr_handler
    mov al, 0x20 ; 0x20 - End of interrupt command
    out 0x20, al ; needs to be send to 0x20 (PIC master command port)
                 ; otherwise the PIC will not send any more interrupts of this type
    iret         ; Return from interrupt

timer_int:
    call timer_intr_handler
    mov al, 0x20
    out 0x20, al
    iret

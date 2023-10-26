#include "print.h"
#include "draw.h"
#include "keyboard.h"

// Read value from IO port 0x60 - keyboard interrupt scancode port
// For more info: https://wiki.osdev.org/Inline_Assembly/Examples#INx
int read_scan_code()
{
    int scan_code;
    asm volatile("inb %1, %0"
                 : "=a"(scan_code)
                 : "Nd"(0x60)
                 : "memory");
    return scan_code;
}

// For information about scan codes see:
// https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
void keyboard_int_handler()
{
    unsigned int scan_code = read_scan_code();

    // Scan codes with extended byte (E0) generate two different interrupts
    // first contains E0 byte, the second one contains the scancode.
    if(scan_code == 0xE0) {
        handle_keyboard_input(read_scan_code());
    }
    else {
        handle_keyboard_input(scan_code);
    }
}

void timer_int_handler() {}

#include "../display/print.h"
#include "../display/draw.h"

#include "keyboard.h"
#include "io.c"

void keyboard_int_handler()
{
    // Read value from IO port 0x60 - keyboard interrupt scancode port
    unsigned int scan_code = inb(0x60);

    // Scan codes with extended byte (E0) generate two different interrupts
    // first contains E0 byte, the second one contains the scancode.
    // For information about scan codes see:
    // https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
    if(scan_code == 0xE0) {
        handle_keyboard_input(inb(0x60));
    }
    else {
        handle_keyboard_input(scan_code);
    }
}

void timer_int_handler() {}




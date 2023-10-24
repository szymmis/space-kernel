#include "print.h"

char c;
void keyboard_int_handler()
{
    int scan_code;
    asm volatile("inb %1, %0"
                 : "=a"(scan_code)
                 : "Nd"(0x60)
                 : "memory");

    unsigned char up = scan_code >> 7 & 0x1;

    if (up)
    {
        put_char(c, 0, 1);
        c++;
    }
}

void timer_int_handler() {}

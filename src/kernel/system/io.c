#include "io.h"

// Read value from an IO port
// For more info: https://wiki.osdev.org/Inline_Assembly/Examples#INx
int inb(int port)
{
    int val;
    asm volatile("inb %1, %0"
                 : "=a"(val)
                 : "Nd"(port)
                 : "memory");
    return val;
}

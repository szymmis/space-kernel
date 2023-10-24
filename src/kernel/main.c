#include "interrupts.c"
#include "draw.c"
#include "print.c"

void main()
{
    print("Hello world", 10, 10);
    print("abcdefghijklmnoprstuwxyz", 10, 20);
    print("ABCDEFGHIJKLMNOPRSTUWXYZ", 10, 30);
    print("1234567890", 10, 40);
    
    printi(12345, 10, 50);
    printi(67890, 35, 50);
}

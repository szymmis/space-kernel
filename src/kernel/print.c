#include "print.h"

void put_char(char c, char x, char y) {
    int offset = (160 * y) + (x * 2);
    VGA_BUFFER[offset] = c;
    VGA_BUFFER[offset + 1] = 0xF;
}

void print(char *str)
{
    char index = 0;
    while (str[index] != '\0')
    {
        put_char(str[index], index, 0);
        index++;
    }
}

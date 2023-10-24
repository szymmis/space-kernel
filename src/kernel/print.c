#include "print.h"

void put_char(char c, char x, char y)
{
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

void printi(int n)
{
    char str[12], i = 0;
    while (n > 0)
    {
        str[i] = '0' + (n % 10);
        n = n / 10;
        i++;
    }

    char length = i;
    char rev_str[length];
    rev_str[length] = '\0';
    while (--i >= 0)
    {
        rev_str[length - 1 - i] = str[i];
    }

    print(rev_str);
}

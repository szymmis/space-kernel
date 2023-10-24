#include "print.h"
#include "draw.h"

void print(char *str, int x, int y)
{
    char index = 0;
    int offset = 0;
    while (str[index] != '\0')
    {
        char c = str[index];
        print_char(c, offset + x, y);
        offset += glyph_width(c);
        index++;
    }
}

void printi(int n, int x, int y)
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

    print(rev_str, x, y);
}

void print_char(char c, int x, int y)
{
    if (c == '0')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '1')
    {
        char glyph[] = {
            0,0,1,0,0,
            0,1,1,0,0,
            0,0,1,0,0,
            0,0,1,0,0,
            0,0,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '2')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            0,0,1,0,0,
            0,1,0,0,0,
            1,1,1,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '3')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            0,0,1,0,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '4')
    {
        char glyph[] = {
            1,0,0,0,0,
            1,0,1,0,0,
            1,1,1,1,0,
            0,0,1,0,0,
            0,0,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '5')
    {
        char glyph[] = {
            1,1,1,1,0,
            1,0,0,0,0,
            1,1,1,0,0,
            0,0,0,1,0,
            1,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '6')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,0,0,
            1,1,1,0,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '7')
    {
        char glyph[] = {
            1,1,1,1,0,
            0,0,0,1,0,
            0,0,1,0,0,
            0,1,0,0,0,
            0,1,0,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '8')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            0,1,1,0,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == '9')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            0,1,1,1,0,
            0,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'a' || c == 'A')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            1,1,1,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'b' || c == 'B')
    {
        char glyph[] = {
            1,1,1,0,0,
            1,0,0,1,0,
            1,1,1,0,0,
            1,0,0,1,0,
            1,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'c' || c == 'C')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            1,0,0,0,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'd' || c == 'D')
    {
        char glyph[] = {
            1,1,1,0,0,
            1,0,0,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
            1,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'e' || c == 'E')
    {
        char glyph[] = {
            1,1,1,1,0,
            1,0,0,0,0,
            1,1,1,0,0,
            1,0,0,0,0,
            1,1,1,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'f' || c == 'F')
    {
        char glyph[] = {
            1,1,1,1,0,
            1,0,0,0,0,
            1,1,1,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'g' || c == 'G')
    {
        char glyph[] = {
            0,1,1,1,0,
            1,0,0,0,0,
            1,0,1,1,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'h' || c == 'H')
    {
        char glyph[] = {
            1,0,0,1,0,
            1,0,0,1,0,
            1,1,1,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'i' || c == 'I')
    {
        char glyph[] = {
            1,0,0,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'j' || c == 'J')
    {
        char glyph[] = {
            1,1,1,1,0,
            0,0,0,1,0,
            0,0,0,1,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'k' || c == 'K')
    {
        char glyph[] = {
            1,0,0,1,0,
            1,0,1,0,0,
            1,1,0,0,0,
            1,0,1,0,0,
            1,0,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'l' || c == 'L')
    {
        char glyph[] = {
            1,0,0,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
            1,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'm' || c == 'M')
    {
        char glyph[] = {
            1,0,0,0,1,
            1,1,0,1,1,
            1,0,1,0,1,
            1,0,0,0,1,
            1,0,0,0,1,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'n' || c == 'N')
    {
        char glyph[] = {
            1,0,0,1,0,
            1,1,0,1,0,
            1,0,1,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'o' || c == 'O')
    {
        char glyph[] = {
            0,1,1,0,0,
            1,0,0,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'p' || c == 'P')
    {
        char glyph[] = {
            1,1,1,0,0,
            1,0,0,1,0,
            1,1,1,0,0,
            1,0,0,0,0,
            1,0,0,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'r' || c == 'R')
    {
        char glyph[] = {
            1,1,1,0,0,
            1,0,0,1,0,
            1,1,1,0,0,
            1,0,1,0,0,
            1,0,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 's' || c == 'S')
    {
        char glyph[] = {
            0,1,1,1,0,
            1,0,0,0,0,
            0,1,1,0,0,
            0,0,0,1,0,
            1,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 't' || c == 'T')
    {
        char glyph[] = {
            1,1,1,1,1,
            0,0,1,0,0,
            0,0,1,0,0,
            0,0,1,0,0,
            0,0,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'u' || c == 'U')
    {
        char glyph[] = {
            1,0,0,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
            1,0,0,1,0,
            0,1,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'w' || c == 'W')
    {
        char glyph[] = {
            1,0,0,0,1,
            1,0,0,0,1,
            1,0,0,0,1,
            1,0,1,0,1,
            0,1,0,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'x' || c == 'X')
    {
        char glyph[] = {
            1,0,0,0,1,
            0,1,0,1,0,
            0,0,1,0,0,
            0,1,0,1,0,
            1,0,0,0,1,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'y' || c == 'Y')
    {
        char glyph[] = {
            1,0,0,0,1,
            0,1,0,1,0,
            0,0,1,0,0,
            0,0,1,0,0,
            0,0,1,0,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
    else if (c == 'z' || c == 'Z')
    {
        char glyph[] = {
            1,1,1,1,0,
            0,0,0,1,0,
            0,0,1,0,0,
            0,1,0,0,0,
            1,1,1,1,0,
        };
        draw_img(glyph, x, y, GLYPH_WIDTH, GLYPH_HEIGHT);
    }
}

int glyph_width(char c) {
    if (c == 'm' || c == 'M' || c == 't' || c == 'T' || c == 'w' || c == 'W' || c == 'x' || c == 'X' || c == 'y' || c == 'Y')
    {
        return GLYPH_WIDTH + 1;
    }
    else if (c == 'l' || c == 'L')
    {
        return 4;
    }
    else if (c == 'i' || c == 'I' || c == ' ')
    {
        return 2;
    }

    return GLYPH_WIDTH;
}

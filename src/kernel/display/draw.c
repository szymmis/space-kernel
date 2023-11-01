#include "draw.h"

void draw_rect(int x, int y, int w, int h)
{
    for (int i = 0; i < h; i++)
    {
        for (int j = 0; j < w; j++)
        {
            VGA_BUFFER[(SCREEN_WIDTH * (i + y)) + (j + x)] = 0x0F;
        }
    }
}

void draw_img(char *img, int x, int y, int w, int h)
{
    for (int i = 0; i < w * h; i++)
    {
        if (img[i] == 1)
        {
            VGA_BUFFER[(SCREEN_WIDTH * ((i / w) + y)) + ((i % w) + x)] = 0x0F;
        }
    }
}

void clear_screen() 
{
    for (int i = 0; i < SCREEN_HEIGHT; i++)
    {
        for (int j = 0; j < SCREEN_WIDTH; j++)
        {
            VGA_BUFFER[(SCREEN_WIDTH * i) + j] = 0x0;
        }
    }
}
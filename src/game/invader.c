#include "../kernel/std/memory.h"
#include "../kernel/display/draw.h"

#include "invader.h"

struct Invader* create_invader(int x, int y, char type) {
    struct Invader* invader = malloc(sizeof(struct Invader));
    invader->x = x;
    invader->y = y;
    invader->dead = 0;
    invader->type = type;
    
    return invader;
}

void draw_invader(struct Invader* i)
{
    if (i->dead == 1) return;

    if(i->type == INVADER)
    {
        char sprite[] = { 
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0,
            0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1,
            1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1,
            0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0,
        };
        
        draw_img(sprite, i->x, i->y, 11, 8);
    }
    else if(i->type == SQUID)
    {
        char sprite[] = { 
            0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
            0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 1, 1, 0, 1, 0, 0,
            0, 1, 0, 1, 0, 0, 1, 0, 1, 0,
        };
        
        draw_img(sprite, i->x, i->y, 10, 8);
    }
    else if(i->type == GOLIATH)
    {
        char sprite[] = { 
            0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0,
            0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0,
            0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0,
        };
        
        draw_img(sprite, i->x, i->y, 12, 8);
    }
}

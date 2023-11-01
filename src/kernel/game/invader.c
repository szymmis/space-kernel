#include "../std/memory.h"
#include "../display/draw.h"

struct Invader
{
    int x;
    int y;
    char dead;
    char type;
};


struct Invader* create_invader(int x, int y, char type) {
    struct Invader* invader = malloc(sizeof(struct Invader*));
    invader->x = x;
    invader->y = y;
    invader->dead = 0;
    invader->type = type;
    
    return invader;
}

void draw_invader(struct Invader* i)
{
    if(i->type == 0)
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
    else if(i->type == 1)
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
    else if(i->type == 2)
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
